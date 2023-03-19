use std::path::Path;
use std::time::Duration;

use anyhow::bail;
use rust_utils::iter::zip_self::ZipSelf;
use crate::audio::resample;
use crate::audio::value_object::SampleRate;

const NB_CHANNEL: usize = 2;

pub struct Sound {
    pub samples: Vec<f32>,
    pub sample_rate: SampleRate,
}

impl Sound {
    pub fn from_wav<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut wav = audrey::open(path)?;
        let desc = wav.description();
        if desc.channel_count() > 2 {
            bail!("Invalid number of channel: {}", desc.channel_count());
        }
        let samples = wav.samples().map(Result::unwrap).collect::<Vec<_>>();
        Ok(Self {
            samples: if desc.channel_count() == 1 { samples.into_iter().zip_self(2).collect() } else { samples },
            sample_rate: SampleRate::try_from(desc.sample_rate() as f32)?,
        })
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs_f32((self.samples.len() / self.nb_channel()) as f32 / self.sample_rate.value())
    }

    #[inline]
    pub fn nb_channel(&self) -> usize {
        NB_CHANNEL // Only stereo sound right now
    }

    fn frame_index_at_time(&self, time: Duration) -> (usize, f32) {
        let index = time.as_secs_f32() * self.sample_rate.value();
        (index as usize, index.fract())
    }

    pub fn nb_frame(&self) -> usize {
        self.samples.len() / self.nb_channel()
    }

    pub fn sample_at_time(&self, time: Duration) -> (f32, f32) {
        if time > self.duration() { return (0.0, 0.0); }
        let (frame_index, _) = self.frame_index_at_time(time);
        (self.samples[frame_index * 2], self.samples[frame_index * 2 + 1])
    }

    pub fn interpolate_at_time(&self, time: Duration) -> (f32, f32) {
        if time > self.duration() { return (0.0, 0.0); }
        let (frame_index, rem) = self.frame_index_at_time(time);
        if frame_index == self.nb_frame() - 1 && let [.., l, r] = self.samples.as_slice() { return (*l, *r); }
        let next_frame_index = frame_index + 1;

        let l1 = self.samples[frame_index * 2];
        let l2 = self.samples[next_frame_index * 2];

        let r1 = self.samples[frame_index * 2 + 1];
        let r2 = self.samples[next_frame_index * 2 + 1];

        let sampled_left = l1 * (1.0 - rem) + l2 * rem;
        let sampled_right = r1 * (1.0 - rem) + r2 * rem;

        (sampled_left, sampled_right)
    }

    pub fn energy(&self) -> f32 {
        let sum = self.samples
            .as_chunks::<2>().0.iter()
            .map(|[l, r]| ((l + r) / 2.0) * ((l + r) / 2.0))
            .sum::<f32>();

        f32::sqrt(sum / (self.samples.len() as f32 / 2.0))
    }

    pub fn resample(&self, target: SampleRate) -> Sound {
        resample(self, target)
    }
}
