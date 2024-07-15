use std::path::Path;

pub use device::Device;
pub use device::Devices;
use eyre::eyre;
use eyre::Context;
use eyre::Ok;
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
    pub sample_rate: f32,
}

pub fn load_samples_from_file<P>(path: P) -> eyre::Result<AudioData>
where
    P: AsRef<Path>,
{
    let mut audio_file = audrey::open(path.as_ref())
        .map_err(|e| eyre!(e))
        .with_context(|| format!("{:?}", path.as_ref()))?;

    let desc = audio_file.description();

    let samples = audio_file
        .samples::<f32>()
        .map(|s| s.map_err(|e| eyre!(e)))
        .collect::<eyre::Result<Vec<f32>>>()?;

    Ok(AudioData {
        samples,
        channel_count: desc.channel_count(),
        sample_rate: desc.sample_rate() as f32,
    })
}
