use std::{path::Path, time::Duration};

use anyhow::bail;
use iter_tools::Itertools;
use rust_utils::iter::zip_self::ZipSelf;

use crate::audio::frame;

#[derive(Clone)]
pub struct StereoSignal {
    pub frames: Vec<(f32, f32)>,
    pub sample_rate: f32,
}

impl StereoSignal {
    pub fn new(duration: Duration, sample_rate: f32) -> StereoSignal {
        StereoSignal {
            frames: vec![(0f32, 0f32); (duration.as_secs_f32() * sample_rate) as usize],
            sample_rate,
        }
    }

    pub fn from_frames(frames: Vec<(f32, f32)>, sample_rate: f32) -> StereoSignal {
        StereoSignal {
            sample_rate,
            frames,
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut audio_file = audrey::open(path)?;
        let desc = audio_file.description();
        if desc.channel_count() > 2 {
            bail!("Invalid number of channel: {}", desc.channel_count());
        }
        let samples = audio_file.samples().map_while(|f| f.ok());
        let samples: Box<dyn Iterator<Item = f32>> = if desc.channel_count() == 1 {
            Box::new(samples.zip_self(2))
        } else {
            Box::new(samples)
        };
        let samples = samples.into_iter().tuples().collect_vec();
        Ok(Self {
            frames: samples,
            sample_rate: desc.sample_rate() as f32,
        })
    }

    fn frame_index_from_duration(&self, duration: Duration) -> (usize, f32) {
        let index = duration.as_secs_f32() * self.sample_rate;
        (index as usize, index.fract())
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs_f32(self.frames.len() as f32 / self.sample_rate)
    }

    pub fn frames_at_duration(&self, duration: Duration) -> anyhow::Result<(f32, f32)> {
        let (frame_index, rem) = self.frame_index_from_duration(duration);

        anyhow::ensure!(
            frame_index < self.frames.len(),
            "Input duration must not exceed signal duration"
        );

        if frame_index == self.frames.len() - 1 {
            if let [.., last_frame] = self.frames.as_slice() {
                return Ok(*last_frame);
            }
        }

        let f1 = self.frames[frame_index];
        let f2 = self.frames[frame_index + 1];

        frame::interpolate(f1, f2, rem)
    }

    pub fn write_frames_at_duration(&mut self, duration: Duration, frames: &[(f32, f32)]) {
        let (frame_index, _) = self.frame_index_from_duration(duration);
        let dest_upper_index = usize::min(self.frames.len() - 1, frame_index + frames.len());
        self.frames[frame_index..dest_upper_index]
            .copy_from_slice(&frames[..(dest_upper_index - frame_index)]);
    }
}
