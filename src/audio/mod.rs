use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
pub use device::Device;

use joy_value_object::{mk_vo, mk_vo_consts};

pub mod device;
pub mod dsp;
pub mod frame;
pub mod player;
pub mod signal;
pub mod synthesis;

mk_vo! {
    pub Volume: f32,
    default: 1.0,
    min: 0.0,
    max: 1.0,
}

impl Volume {
    pub fn db(self) -> Decibels {
        if self.value() == 0.0 {
            Decibels::MIN
        } else {
            Decibels::new_unchecked(10.0 * f32::log10(self.value()))
        }
    }
}

mk_vo! {
    pub Decibels: f32,
    default: -3.0,
    min: -30.0,
    max: 0.0,
}

impl Decibels {
    pub fn volume(self) -> Volume {
        if (self.value() - Decibels::MIN_VALUE).abs() < 0.1f32 {
            Volume::MIN
        } else {
            Volume::new_clamped(10.0f32.powf(self.value() / 10.0))
        }
    }
}

mk_vo! {
    pub Pan: f32,
    default: 0.0,
    min: -1.0,
    max: 1.0,
}

mk_vo_consts! {
    Pan,
    LEFT => Pan::MIN_VALUE,
    RIGHT => Pan::MAX_VALUE,
}

impl Pan {
    pub fn left_volume(&self) -> Volume {
        Volume::new_unchecked(1.0 - self.value().clamp(0.0, 1.0))
    }

    pub fn right_volume(&self) -> Volume {
        Volume::new_unchecked(1.0 + self.value().clamp(-1.0, 0.0))
    }
}

#[derive(Debug)]
pub struct AudioData {
    pub samples: Vec<f32>,
    pub channel_count: u32,
    pub frame_rate: f32,
}

pub fn load_samples_from_file<P>(path: P) -> anyhow::Result<AudioData>
where
    P: AsRef<Path>,
{
    let mut audio_file = audrey::open(path.as_ref())
        .map_err(|e| anyhow!(e))
        .with_context(|| format!("{:?}", path.as_ref()))?;

    let desc = audio_file.description();

    let samples = audio_file
        .samples::<f32>()
        .map(|s| s.map_err(|e| anyhow!(e)))
        .collect::<anyhow::Result<Vec<f32>>>()?;

    Ok(AudioData {
        samples,
        channel_count: desc.channel_count(),
        frame_rate: desc.sample_rate() as f32,
    })
}
