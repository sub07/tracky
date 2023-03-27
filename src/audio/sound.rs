use std::path::Path;
use std::time::Duration;

use anyhow::bail;
use rust_utils::iter::zip_self::ZipSelf;

use crate::audio::{resample, SampleRate};

#[derive(Clone)]
pub struct Sound {
    pub frames: Vec<(f32, f32)>,
    pub sample_rate: SampleRate,
}

pub struct PitchShiftedSoundIterator<'a> {
    sound: &'a Sound,
    duration_increment: Duration,
    current_duration: Duration,
    src_duration: Duration,
    output_size: usize,
}

impl<'a> PitchShiftedSoundIterator<'a> {
    pub fn new(sound: &'a Sound, multiplier: f32) -> PitchShiftedSoundIterator<'a> {
        let duration_increment = Duration::from_secs_f32((multiplier * sound.duration().as_secs_f32()) / sound.frames.len() as f32);

        PitchShiftedSoundIterator {
            sound,
            duration_increment,
            current_duration: Duration::ZERO,
            src_duration: sound.duration(),
            output_size: (sound.frames.len() as f32 / multiplier) as usize,
        }
    }
}

impl Iterator for PitchShiftedSoundIterator<'_> {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_duration >= self.src_duration { return None; }
        let frame = self.sound.interpolate_frame_at_time(self.current_duration);
        self.current_duration += self.duration_increment;
        Some(frame)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.output_size, Some(self.output_size))
    }
}

impl Sound {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut audio_file = audrey::open(path)?;
        let desc = audio_file.description();
        if desc.channel_count() > 2 {
            bail!("Invalid number of channel: {}", desc.channel_count());
        }
        let samples = audio_file.samples().map(Result::unwrap).collect::<Vec<f32>>();
        let samples = if desc.channel_count() == 1 {
            samples.into_iter().zip_self(2).collect::<Vec<_>>()
        } else {
            samples
        };
        let samples = samples.into_iter().array_chunks::<2>().map(|[l, r]| (l, r)).collect();
        Ok(Self {
            frames: samples,
            sample_rate: SampleRate::try_from(desc.sample_rate() as f32)?,
        })
    }

    pub fn from_frames(frames: Vec<(f32, f32)>, sample_rate: SampleRate) -> Sound {
        Sound {
            frames,
            sample_rate,
        }
    }

    pub fn from_duration(duration: Duration, sample_rate: SampleRate) -> Self {
        let nb_frame = duration.as_secs_f32() * sample_rate.value();
        Sound {
            frames: vec![(0.0, 0.0); nb_frame.round() as usize],
            sample_rate,
        }
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs_f32(self.frames.len() as f32 / self.sample_rate.value())
    }

    fn frame_index_at_time(&self, time: Duration) -> (usize, f32) {
        let index = time.as_secs_f32() * self.sample_rate.value();
        (index as usize, index.fract())
    }

    pub fn interpolate_frame_at_time(&self, time: Duration) -> (f32, f32) {
        if time > self.duration() { return (0.0, 0.0); }
        let (frame_index, rem) = self.frame_index_at_time(time);
        if frame_index == self.frames.len() - 1 && let [.., last_frame] = self.frames.as_slice() { return *last_frame; }

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

    pub fn energy(&self) -> f32 {
        let sum = self.frames.iter()
            .map(|(l, r)| ((l + r) / 2.0) * ((l + r) / 2.0))
            .sum::<f32>();

        f32::sqrt(sum / (self.frames.len() as f32 / 2.0))
    }

    pub fn resample(&self, target: SampleRate) -> Sound {
        resample(self, target)
    }

    pub fn pitch_shift_iter(&self, multiplier: f32) -> PitchShiftedSoundIterator<'_> {
        PitchShiftedSoundIterator::new(self, multiplier)
    }

    pub fn pitch_shifted(&self, multiplier: f32) -> Sound {
        let frames = self.pitch_shift_iter(multiplier).collect::<Vec<_>>();
        Sound::from_frames(frames, self.sample_rate)
    }
}
