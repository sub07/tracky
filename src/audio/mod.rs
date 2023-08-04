use iter_tools::Itertools;
use rust_utils::iter::zip_self::ZipSelf;
use std::path::Path;

use anyhow::bail;

pub struct PcmStereoSample {
    pub frames: Vec<(f32, f32)>,
    pub sample_rate: f32,
}

impl PcmStereoSample {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut audio_file = audrey::open(path)?;
        let desc = audio_file.description();
        if desc.channel_count() > 2 {
            bail!("Invalid number of channel: {}", desc.channel_count());
        }
        let samples = audio_file.samples().map(Result::unwrap).collect_vec();
        let samples = if desc.channel_count() == 1 {
            samples.into_iter().zip_self(2).collect::<Vec<_>>()
        } else {
            samples
        };
        let samples = samples
            .into_iter()
            .array_chunks::<2>()
            .map(|[l, r]| (l, r))
            .collect();
        Ok(Self {
            frames: samples,
            sample_rate: desc.sample_rate() as f32,
        })
    }
}
