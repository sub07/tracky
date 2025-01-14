use std::sync::mpsc::{Receiver, Sender};

use anyhow::bail;
use builder_pattern::Builder;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig,
};

use joy_error::OptionToResultExt;
use log::{error, info};

use crate::{
    event::Event,
    model::song::{self},
};

pub struct AudioPlayer {
    pub frame_rate: f32,
    stream: Stream,
}

#[rustfmt::skip]
#[derive(Builder)]
pub struct AudioPlayerBuilder {
    #[default(None)]
    pub device: Option<crate::audio::Device>,
    pub initial_state: song::State,
    pub state_event_rx: Receiver<song::Event>,
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
            "Audio playing on {} at {}Hz",
            device.name,
            sample_rate,
        );

        Ok(AudioPlayer {
            frame_rate: sample_rate,
            stream,
        })
    }
}

fn create_stream<SampleType>(
    device: Device,
    config: StreamConfig,
    mut state: song::State,
    state_event_rx: Receiver<song::Event>,
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
    state: &mut song::State,
    state_event_rx: &Receiver<song::Event>,
    event_tx: Sender<Event>,
) where
    SampleType: Sample + FromSample<f32>,
{
    out.fill(SampleType::from_sample(0.0));
}
