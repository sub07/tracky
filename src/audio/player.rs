use std::{
    marker::PhantomData,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use anyhow::ensure;
use builder_pattern::Builder;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, SampleFormat, SampleRate, Stream,
};

use joy_error::OptionToResultExt;
use log::{error, info};

pub enum ProcessStatus {
    Ongoing,
    Finished,
}

pub trait Process<State, Event> {
    fn frame_callback(
        &mut self,
        out: &mut [f32],
        state: &mut State,
        events: &[Event],
    ) -> ProcessStatus;
}

impl<F, S, E> Process<S, E> for F
where
    F: FnMut(&mut [f32], &mut S, &[E]) -> ProcessStatus,
{
    fn frame_callback(&mut self, out: &mut [f32], state: &mut S, events: &[E]) -> ProcessStatus {
        self(out, state, events)
    }
}

pub struct AudioPlayer<S, E> {
    pub name: String,
    pub frame_rate: f32,
    stop_tx: Sender<()>,
    _marker: PhantomData<(S, E)>,
}

#[derive(Builder)]
struct AudioPlayerBuilder<S, E, P: Process<S, E>> {
    #[default("Unnamed audio player".into())]
    name: String,
    device: Option<crate::audio::Device>,
    processor: P,
    initial_state: S,
    event_rx: Receiver<E>,
}

impl<S, E, P> AudioPlayerBuilder<S, E, P>
where
    S: Send + 'static,
    E: Send + 'static,
    P: Process<S, E> + Send + 'static,
{
    pub fn into_player(self) -> anyhow::Result<AudioPlayer<S, E>> {
        let device = self
            .device
            .unwrap_or_fallible(|| crate::audio::Devices::default_output())?;

        let (creation_tx, creation_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel();

        let crate::audio::device::Device(device_name, cpal_device) = device;

        let audio_player_name = self.name.clone();

        thread::Builder::new()
            .name(format!(
                "'{}' audio player command handler",
                self.name.clone()
            ))
            .spawn(move || {
                audio_stream_thread(
                    cpal_device,
                    creation_tx,
                    stop_rx,
                    audio_player_name,
                    self.processor,
                    self.initial_state,
                    self.event_rx,
                );
            })?;

        let sample_rate = creation_rx.recv()??.0 as f32;
        info!(
            "Audio player '{}' playing on {device_name} at {}Hz",
            self.name.clone(),
            sample_rate
        );

        Ok(AudioPlayer {
            name: self.name.clone(),
            frame_rate: sample_rate,
            stop_tx,
            _marker: PhantomData,
        })
    }
}
#[inline]
fn audio_stream_thread<S, E, P>(
    cpal_device: Device,
    creation_tx: Sender<anyhow::Result<SampleRate>>,
    stop_rx: Receiver<()>,
    audio_player_name: String,
    processor: P,
    state: S,
    event_rx: Receiver<E>,
) where
    S: Send + 'static,
    E: Send + 'static,
    P: Process<S, E> + Send + 'static,
{
    match init_stream(cpal_device, processor, state, event_rx) {
        Ok((stream, sample_rate)) => {
            creation_tx.send(Ok(sample_rate)).unwrap();
            if let Err(err) = stream.play() {
                error!("{err:?} when trying to start playback on player '{audio_player_name}'");
                return;
            }
            stop_rx.recv().unwrap();
            info!("Stopping '{audio_player_name}' audio player");
        }
        Err(e) => creation_tx.send(Err(e)).unwrap(),
    }
}

impl<S, E> AudioPlayer<S, E> {
    pub fn stop(&mut self) -> anyhow::Result<()> {
        self.stop_tx.send(())?;
        Ok(())
    }
}

fn init_stream<S, E, P>(
    device: Device,
    mut processor: P,
    mut state: S,
    event_rx: Receiver<E>,
) -> anyhow::Result<(Stream, SampleRate)>
where
    S: Send + 'static,
    E: Send + 'static,
    P: Process<S, E> + Send + 'static,
{
    let config = device.default_output_config()?;

    let sample_rate = config.sample_rate();

    ensure!(config.channels() == 2);
    ensure!(config.sample_format() == SampleFormat::F32);

    let mut events = Vec::with_capacity(100);

    let stream = device.build_output_stream(
        &config.into(),
        move |out: &mut [f32], _| {
            events.clear();
            events.extend(event_rx.try_iter());
            processor.frame_callback(out, &mut state, events.as_slice());
        },
        |e| error!("Cannot start audio stream: {e:?}"),
        None,
    )?;

    Ok((stream, sample_rate))
}
