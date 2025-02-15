use std::sync::mpsc::Receiver;

use anyhow::bail;
use builder_pattern::Builder;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig,
};

use log::{error, info, warn};

use crate::{
    assert_log_fail,
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
    pub state_event_rx: Receiver<model::Command>,
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
    state_event_rx: Receiver<model::Command>,
    event_tx: EventSender,
) -> anyhow::Result<(Stream, f32)>
where
    SampleType: SizedSample + FromSample<f32>,
{
    let frame_rate = config.sample_rate.0 as f32;

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

    Ok((stream, frame_rate))
}

fn audio_callback<SampleType>(
    out: &mut [SampleType],
    state: &mut model::State,
    state_command_rx: &Receiver<model::Command>,
    event_tx: EventSender,
) where
    SampleType: Sample + FromSample<f32>,
{
    macro_rules! update_state {
        ($command:expr) => {
            state.handle_command($command);
            if let Err(e) = event_tx.send_event(Event::AudioCallback($command)) {
                error!("Event channel broken while sending {:?}: {e}", $command);
            }
        };
    }

    out.fill(SampleType::from_sample(0.0));

    for command in state_command_rx.try_iter() {
        state.handle_command(command);
    }

    if let Some(old_sample_count) = state
        .step_output
        .as_ref()
        .filter(|signal| signal.as_ref().sample_count() != out.len())
        .map(|signal| signal.as_ref().sample_count())
    {
        warn!(
            "Heap allocations triggered in audio thread: playback sample count changed from {} to {}",
            old_sample_count,
            out.len(),
        );
        update_state!(model::Command::UpdatePlaybackSampleCount(out.len()));
    }

    update_state!(model::Command::PerformPlaybacksStep);

    match state.output_samples() {
        Ok(frames) => {
            for (out, produced_sample) in out.iter_mut().zip(frames.samples()) {
                *out = Sample::from_sample(produced_sample);
            }
        }
        Err(err) => {
            assert_log_fail!("Error while reading produced sample from audio callback: {err:?}")
        }
    }
}
