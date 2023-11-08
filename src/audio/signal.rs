use std::{
    cell::OnceCell,
    path::Path,
    sync::{Mutex, OnceLock},
    time::Duration,
};

use anyhow::bail;
use iter_tools::Itertools;
use rust_utils::iter::zip_self::ZipSelf;

use crate::{
    audio::frame,
    model::field::{value_object::OctaveValue, Note, NoteName},
};

use super::{value_object::*, FrameIterator, IntoFrequency};

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

    pub fn write_frames_at_duration(
        &mut self,
        duration: Duration,
        signal: &StereoSignal,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.sample_rate == signal.sample_rate,
            "The two signal have different sample rate"
        );
        let (frame_index, _) = self.frame_index_from_duration(duration);
        let dest_upper_index = usize::min(self.frames.len() - 1, frame_index + signal.frames.len());
        self.frames[frame_index..dest_upper_index]
            .copy_from_slice(&signal.frames[..(dest_upper_index - frame_index)]);
        Ok(())
    }

    pub fn write_signal_to_disk(&self, file_name: String) -> anyhow::Result<()> {
        let mut bytes = Vec::new();
        for (l, r) in self.frames.iter() {
            bytes.extend(l.to_le_bytes());
            bytes.extend(r.to_le_bytes());
        }

        let temp_file_name = format!("{}-temp.raw", file_name);
        std::fs::write(&temp_file_name, &bytes)?;

        std::process::Command::new("C:\\Users\\mpardo\\AppData\\Local\\Microsoft\\WinGet\\Packages\\Gyan.FFmpeg_Microsoft.Winget.Source_8wekyb3d8bbwe\\ffmpeg-6.0-full_build\\bin\\ffmpeg.exe")
        .arg("-f").arg("f32le")
        .arg("-ar").arg(format!("{}", self.sample_rate as u32))
        .arg("-ac").arg("2")
        .arg("-i").arg(&temp_file_name)
        .arg(file_name)
        .output().unwrap();

        std::fs::remove_file(&temp_file_name)?;

        Ok(())
    }
}

impl std::ops::AddAssign<&StereoSignal> for StereoSignal {
    fn add_assign(&mut self, rhs: &StereoSignal) {
        for ((l1, r1), (l2, r2)) in self.frames.iter_mut().zip(rhs.frames.iter()) {
            *l1 += *l2;
            *r1 += *r2;
        }
    }
}

impl<'a> From<&'a StereoSignal> for &'a [(f32, f32)] {
    fn from(value: &'a StereoSignal) -> Self {
        &value.frames
    }
}

impl FrameIterator for StereoSignal {
    fn next(
        &mut self,
        freq: f32,
        amp: Volume,
        pan: Pan,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<(f32, f32)> {
        static C5_FREQ: OnceLock<f32> = OnceLock::new();

        let mut p = *phase;
        let (l, r) = self
            .frames_at_duration(Duration::from_secs_f32(p / sample_rate))
            .ok()?;

        let c5_freq =
            C5_FREQ.get_or_init(|| (NoteName::C, OctaveValue::new(5).unwrap()).into_frequency());

        p += freq / c5_freq;

        *phase = p;

        let left_amp = amp.value() * pan.left().value();
        let right_amp = amp.value() * pan.right().value();
        Some((l * left_amp, r * right_amp))
    }
}
