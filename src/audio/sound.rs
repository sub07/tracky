use std::path::Path;
use std::time::Duration;

use anyhow::bail;
use rust_utils::iter::zip_self::ZipSelf;

const NB_CHANNEL: usize = 2;

pub struct Sound {
    pub samples: Vec<f32>,
    pub speed: f64,
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
            speed: desc.sample_rate() as f64,
        })
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs_f64((self.samples.len() / self.nb_channel()) as f64 / self.speed as f64)
    }

    #[inline]
    pub fn nb_channel(&self) -> usize {
        NB_CHANNEL // Only stereo sound right now
    }

    fn frame_index_at_time(&self, time: Duration) -> usize {
        (time.as_secs_f64() * self.speed) as usize
    }

    pub fn sample_at_time(&self, time: Duration) -> (f32, f32) {
        if time > self.duration() { return (0.0, 0.0); }
        let frame_index = self.frame_index_at_time(time);
        (self.samples[frame_index * 2], self.samples[frame_index * 2 + 1])
    }
}
