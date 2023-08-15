use iter_tools::Itertools;
use rust_utils::iter::zip_self::ZipSelf;
use std::{path::Path, time::Duration};

use anyhow::bail;

#[derive(Clone)]
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
            samples.into_iter().zip_self(2).collect_vec()
        } else {
            samples
        };
        let samples = samples.into_iter().tuples().collect_vec();
        Ok(Self {
            frames: samples,
            sample_rate: desc.sample_rate() as f32,
        })
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs_f32(self.frames.len() as f32 / self.sample_rate)
    }

    pub fn interpolate_frame_at_time(&self, time: Duration) -> (f32, f32) {
        if time > self.duration() { return (0.0, 0.0); }
        let (frame_index, rem) = self.frame_index_at_time(time);
        if frame_index == self.frames.len() - 1 { 
            if let [.., last_frame] = self.frames.as_slice() {
                return *last_frame; 
            }
        }

        let (l1, r1) = self.frames[frame_index];
        let (l2, r2) = self.frames[frame_index + 1];

        let interpolated_left = l1 * (1.0 - rem) + l2 * rem;
        let interpolated_right = r1 * (1.0 - rem) + r2 * rem;

        (interpolated_left, interpolated_right)
    }

    pub fn set_frame_at_time(&mut self, time: Duration, frame: (f32, f32)) {
        let (frame_index, _) = self.frame_index_at_time(time);
        self.frames[frame_index] = frame;
    }

    fn frame_index_at_time(&self, time: Duration) -> (usize, f32) {
        let index = time.as_secs_f32() * self.sample_rate;
        (index as usize, index.fract())
    }

}
