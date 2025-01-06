use std::{
    marker::PhantomData,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use anyhow::{anyhow, ensure};
use builder_pattern::Builder;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, SampleFormat, SampleRate, Stream,
};

use joy_error::OptionToResultExt;
use log::{error, info};

pub trait Process<State, StateEvent, PlaybackEvent> {
    fn frame_callback(
        &self,
        out: &mut [f32],
        frame_rate: f32,
        state: &mut State,
        events: &[StateEvent],
        playback_event: Sender<PlaybackEvent>,
    );
}

impl<F, State, StateEvent, PlaybackEvent> Process<State, StateEvent, PlaybackEvent> for F
where
    F: Fn(&mut [f32], f32, &mut State, &[StateEvent], Sender<PlaybackEvent>),
{
    fn frame_callback(
        &self,
        out: &mut [f32],
        frame_rate: f32,
        state: &mut State,
        state_events: &[StateEvent],
        playback_event: Sender<PlaybackEvent>,
    ) {
        self(out, frame_rate, state, state_events, playback_event);
    }
}

pub struct AudioPlayer {
    pub name: String,
    pub frame_rate: f32,
    stream: Stream,
}

#[rustfmt::skip]
#[derive(Builder)]
pub struct AudioPlayerBuilder<
    State,
    StateEvent,
    PlaybackEvent,
    P: Process<State, StateEvent, PlaybackEvent>
> {
    #[into]
    #[default("Unnamed audio player".into())]
    pub name: String,
    #[default(None)]
    pub device: Option<crate::audio::Device>,
    pub processor: P,
    pub initial_state: State,
    pub state_event_rx: Receiver<StateEvent>,
    pub playback_event_tx: Sender<PlaybackEvent>,
}

impl<State, StateEvent, PlaybackEvent, P> AudioPlayerBuilder<State, StateEvent, PlaybackEvent, P>
where
    State: Send + 'static,
    StateEvent: Send + 'static,
    PlaybackEvent: Send + 'static,
    P: Process<State, StateEvent, PlaybackEvent> + Send + 'static,
{
    pub fn into_player(self) -> anyhow::Result<AudioPlayer> {
        let device = self
            .device
            .unwrap_or_fallible(crate::audio::device::default_output)?;

        let (stream, sample_rate) = create_stream(
            device.inner,
            self.processor,
            self.initial_state,
            self.state_event_rx,
            self.playback_event_tx,
        )?;

        info!(
            "Audio player '{}' playing on {} at {}Hz",
            self.name.clone(),
            device.name,
            sample_rate,
        );

        Ok(AudioPlayer {
            name: self.name,
            frame_rate: sample_rate,
            stream,
        })
    }
}

fn create_stream<State, StateEvent, PlaybackEvent, P>(
    device: Device,
    processor: P,
    mut state: State,
    state_event_rx: Receiver<StateEvent>,
    playback_event_tx: Sender<PlaybackEvent>,
) -> anyhow::Result<(Stream, f32)>
where
    State: Send + 'static,
    StateEvent: Send + 'static,
    PlaybackEvent: Send + 'static,
    P: Process<State, StateEvent, PlaybackEvent> + Send + 'static,
{
    let config = device.default_output_config()?;

    let sample_rate = config.sample_rate().0 as f32;

    assert!(config.channels() == 2);
    assert!(config.sample_format() == SampleFormat::F32);

    let mut events = Vec::with_capacity(100);

    let stream = device.build_output_stream(
        &config.into(),
        move |out: &mut [f32], _| {
            events.clear();
            events.extend(state_event_rx.try_iter());
            processor.frame_callback(
                out,
                sample_rate,
                &mut state,
                events.as_slice(),
                playback_event_tx.clone(),
            );
        },
        |e| error!("Cannot start audio stream: {e:?}"),
        None,
    )?;

    stream.play()?;

    Ok((stream, sample_rate))
}
