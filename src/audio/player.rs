use std::{
    f32::consts::PI,
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use anyhow::bail;
use builder_pattern::Builder;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig,
};

use joy_error::OptionToResultExt;
use log::{error, info};

use crate::{event::Event, keybindings::Action};

pub struct SineState {
    pub freq: f32,
    pub phase: f32,
    pub time_played: Duration,
}
pub type StateEvent = String;

pub struct AudioPlayer {
    pub name: String,
    pub frame_rate: f32,
    stream: Stream,
}

#[rustfmt::skip]
#[derive(Builder)]
pub struct AudioPlayerBuilder {
    #[into]
    #[default("Unnamed audio player".into())]
    pub name: String,
    #[default(None)]
    pub device: Option<crate::audio::Device>,
    pub initial_state: SineState,
    pub state_event_rx: Receiver<StateEvent>,
    pub event_tx: Sender<Event>,
}

impl AudioPlayerBuilder {
    pub fn into_player(self) -> anyhow::Result<AudioPlayer> {
        let device = self
            .device
            .unwrap_or_fallible(crate::audio::device::default_output)?;

        let config = device.inner.default_output_config()?;

        let (stream, sample_rate) = match config.sample_format() {
            SampleFormat::I8 => create_stream::<i8>(
                device.inner,
                config.into(),
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::I16 => create_stream::<i16>(
                device.inner,
                config.into(),
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::I32 => create_stream::<i32>(
                device.inner,
                config.into(),
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::I64 => create_stream::<i64>(
                device.inner,
                config.into(),
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::U8 => create_stream::<u8>(
                device.inner,
                config.into(),
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::U16 => create_stream::<u16>(
                device.inner,
                config.into(),
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::U32 => create_stream::<u32>(
                device.inner,
                config.into(),
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::U64 => create_stream::<u64>(
                device.inner,
                config.into(),
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::F32 => create_stream::<f32>(
                device.inner,
                config.into(),
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::F64 => create_stream::<f64>(
                device.inner,
                config.into(),
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            sample_format => bail!("Unsupported sample format '{sample_format}'"),
        };

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

fn create_stream<SampleType>(
    device: Device,
    config: StreamConfig,
    mut state: SineState,
    state_event_rx: Receiver<StateEvent>,
    event_tx: Sender<Event>,
) -> anyhow::Result<(Stream, f32)>
where
    SampleType: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;

    assert!(config.channels == 2);

    let stream = device.build_output_stream(
        &config,
        move |out: &mut [SampleType], _| {
            audio_callback(
                out,
                sample_rate,
                &mut state,
                &state_event_rx,
                event_tx.clone(),
            );
        },
        |e| error!("Cannot start audio stream: {e:?}"),
        None,
    )?;

    stream.play()?;

    Ok((stream, sample_rate))
}

fn audio_callback<SampleType>(
    out: &mut [SampleType],
    frame_rate: f32,
    state: &mut SineState,
    state_event_rx: &Receiver<StateEvent>,
    event_tx: Sender<Event>,
) where
    SampleType: Sample + FromSample<f32>,
{
    while let Ok(state_event) = state_event_rx.try_recv() {
        match state_event.as_str() {
            "up" => state.freq += 20.0,
            "down" => state.freq -= 20.0,
            "done" => event_tx.send(Event::Action(Action::TogglePlay)).unwrap(),
            _ => {}
        }
    }

    let buffer_duration = Duration::from_secs_f32(out.len() as f32 / 2.0 / frame_rate);
    state.time_played += buffer_duration;

    if state.time_played > Duration::from_secs(3) {
        event_tx.send(Event::Action(Action::TogglePlay)).unwrap();
    }

    for out in out.chunks_exact_mut(2) {
        state.phase += 2.0 * PI * state.freq * (1.0 / frame_rate);
        let s = state.phase.sin() * 0.1;
        out[0] = SampleType::from_sample(s);
        out[1] = out[0];
    }
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test_player() {
        let (state_event_tx, state_event_rx) = std::sync::mpsc::channel();
        let (playback_event_tx, event_rx) = std::sync::mpsc::channel();

        let player = AudioPlayerBuilder::new()
            .name("Test player")
            .initial_state(SineState {
                freq: 440.0,
                phase: 0.0,
                time_played: Duration::ZERO,
            })
            .state_event_rx(state_event_rx)
            .event_tx(playback_event_tx)
            .build()
            .into_player()
            .unwrap();

        std::thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            state_event_tx.send("up".to_string()).unwrap();
            thread::sleep(Duration::from_secs_f32(0.5));
            state_event_tx.send("up".to_string()).unwrap();
            thread::sleep(Duration::from_secs_f32(0.5));
            state_event_tx.send("up".to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
            state_event_tx.send("down".to_string()).unwrap();
            thread::sleep(Duration::from_secs_f32(0.5));
            state_event_tx.send("down".to_string()).unwrap();
            thread::sleep(Duration::from_secs_f32(0.5));
            state_event_tx.send("down".to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
            state_event_tx.send("done".to_string()).unwrap();
        });

        loop {
            let event = event_rx.recv().unwrap();
            if let Event::Action(Action::ExitApp) = event {
                break;
            }
        }
    }
}
