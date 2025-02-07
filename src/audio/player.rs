use std::sync::mpsc::Receiver;

use anyhow::bail;
use builder_pattern::Builder;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig,
};

use joy_vector::Vector;
use log::{error, info, warn};

use crate::{
    event::Event,
    model::{self},
    EventSender,
};

use super::device::ConfiguredDevice;

pub struct AudioPlayer {
    pub frame_rate: f32,
    _stream: Stream,
}

#[rustfmt::skip]
#[derive(Builder)]
pub struct AudioPlayerBuilder {
    pub device: ConfiguredDevice,
    pub initial_state: model::State,
    pub state_event_rx: Receiver<model::Event>,
    pub event_tx: EventSender,
}

impl AudioPlayerBuilder {
    pub fn into_player(self) -> anyhow::Result<AudioPlayer> {
        let (stream, sample_rate) = match self.device.sample_format {
            SampleFormat::I8 => create_stream::<i8>(
                self.device.inner,
                self.device.config,
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::I16 => create_stream::<i16>(
                self.device.inner,
                self.device.config,
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::I32 => create_stream::<i32>(
                self.device.inner,
                self.device.config,
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::I64 => create_stream::<i64>(
                self.device.inner,
                self.device.config,
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::U8 => create_stream::<u8>(
                self.device.inner,
                self.device.config,
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::U16 => create_stream::<u16>(
                self.device.inner,
                self.device.config,
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::U32 => create_stream::<u32>(
                self.device.inner,
                self.device.config,
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::U64 => create_stream::<u64>(
                self.device.inner,
                self.device.config,
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::F32 => create_stream::<f32>(
                self.device.inner,
                self.device.config,
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            SampleFormat::F64 => create_stream::<f64>(
                self.device.inner,
                self.device.config,
                self.initial_state,
                self.state_event_rx,
                self.event_tx,
            )?,
            sample_format => bail!("Unsupported sample format '{sample_format}'"),
        };

        info!(
            "Audio player up and running on [{}] {} at {}Hz",
            self.device.host_name, self.device.name, sample_rate,
        );

        Ok(AudioPlayer {
            frame_rate: sample_rate,
            _stream: stream,
        })
    }
}

fn create_stream<SampleType>(
    device: Device,
    config: StreamConfig,
    mut state: model::State,
    state_event_rx: Receiver<model::Event>,
    event_tx: EventSender,
) -> anyhow::Result<(Stream, f32)>
where
    SampleType: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;

    assert!(config.channels == 2);

    let event_tx_error = event_tx.clone();

    let stream = device.build_output_stream(
        &config,
        move |out: &mut [SampleType], _| {
            audio_callback(out, &mut state, &state_event_rx, event_tx.clone());
        },
        move |e| {
            event_tx_error
                .send_event(Event::StopAudioPlayer(Some(e.into())))
                .unwrap();
        },
        None,
    )?;

    stream.play()?;

    Ok((stream, sample_rate))
}

fn audio_callback<SampleType>(
    out: &mut [SampleType],
    state: &mut model::State,
    state_event_rx: &Receiver<model::Event>,
    event_tx: EventSender,
) where
    SampleType: Sample + FromSample<f32>,
{
    macro_rules! update_state {
        ($event:expr) => {
            state.handle_event($event);
            if let Err(e) = event_tx.send_event(Event::AudioCallback($event)) {
                error!("Event channel broken: {e}");
            }
        };
    }

    out.fill(SampleType::from_sample(0.0));

    for event in state_event_rx.try_iter() {
        state.handle_event(event);
    }

    if let Some(old_sample_count) = state
        .playback
        .as_ref()
        .filter(|playback| playback.step_signal.as_ref().sample_count() != out.len())
        .map(|playback| playback.step_signal.as_ref().sample_count())
    {
        warn!(
            "Heap allocations triggered in audio thread: playback sample count changed from {} to {}",
            old_sample_count,
            out.len(),
        );
        update_state!(model::Event::UpdatePlaybackSampleCount(out.len()));
    }

    if state.is_playing() {
        update_state!(model::Event::PerformStepPlayback);

        if let Some(playback) = state.playback.as_ref() {
            for (out, Vector([left, right])) in out[..playback.last_step_computed_sample_count]
                .chunks_mut(2)
                .zip(
                    playback
                        .step_signal
                        .iter()
                        .take(playback.last_step_computed_sample_count / 2),
                )
            {
                out[0] = SampleType::from_sample(*left);
                out[1] = SampleType::from_sample(*right);
            }
        }

        if state.is_playback_done() {
            update_state!(model::Event::StopSongPlayback);
        }
    }
}
