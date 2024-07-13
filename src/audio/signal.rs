use std::{
    ops::{Deref, DerefMut},
    path::Path,
    time::Duration,
};

use eyre::{bail, ensure};
use itertools::Itertools;
use joy_iter::zip_self::ZipSelf;
use joy_vector::Vector;

use crate::audio::dsp;

use super::{frame::Frame, load_samples_from_file};

pub type StereoSignal = Signal<2>;

#[derive(Clone)]
pub struct Signal<const FRAME_SIZE: usize> {
    pub frames: Vec<Frame<FRAME_SIZE>>,
    pub sample_rate: f32,
}

impl<const FRAME_SIZE: usize> Signal<FRAME_SIZE> {
    pub fn new(duration: Duration, sample_rate: f32) -> Self {
        Signal {
            frames: vec![Frame::default(); (duration.as_secs_f32() * sample_rate) as usize],
            sample_rate,
        }
    }

    pub fn from_frames(frames: Vec<Frame<FRAME_SIZE>>, sample_rate: f32) -> Self {
        Signal {
            frames,
            sample_rate,
        }
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs_f32(self.frames.len() as f32 / self.sample_rate)
    }

    fn frame_index_from_duration(&self, duration: Duration) -> (usize, f32) {
        let index = duration.as_secs_f32() * self.sample_rate;
        (index as usize, index.fract())
    }

    pub fn write_signal_at_duration(
        &mut self,
        duration: Duration,
        signal: &Self,
    ) -> eyre::Result<()> {
        ensure!(
            self.sample_rate == signal.sample_rate,
            "The two signal must have the same sample rate"
        );
        let (copy_start_index, _) = self.frame_index_from_duration(duration);
        let copy_end_index = usize::min(
            self.frames.len() - 1,
            copy_start_index + signal.frames.len(),
        );
        self.frames[copy_start_index..copy_end_index]
            .copy_from_slice(&signal.frames[..(copy_end_index - copy_start_index)]);
        Ok(())
    }

    pub fn append_signal(mut self, signal: &Signal<FRAME_SIZE>) -> eyre::Result<Self> {
        ensure!(
            self.sample_rate == signal.sample_rate,
            "The two signal must have the same sample rate"
        );
        self.frames.extend(signal.frames.iter());
        Ok(self)
    }

    pub unsafe fn into_samples(self) -> Vec<f32> {
        let (ptr, len, cap) = self.frames.into_raw_parts();
        Vec::from_raw_parts(ptr as *mut f32, len * FRAME_SIZE, cap * FRAME_SIZE)
    }

    pub unsafe fn from_samples(samples: Vec<f32>, sample_rate: f32) -> eyre::Result<Self> {
        ensure!(samples.len() % FRAME_SIZE == 0);
        let (ptr, len, cap) = samples.into_raw_parts();
        let frames = Vec::from_raw_parts(
            ptr as *mut Frame<FRAME_SIZE>,
            len / FRAME_SIZE,
            cap / FRAME_SIZE,
        );
        Ok(Signal::from_frames(frames, sample_rate))
    }

    pub fn lerp_frame_at_duration(&self, duration: Duration) -> eyre::Result<Frame<FRAME_SIZE>> {
        let (frame_index, rem) = self.frame_index_from_duration(duration);

        ensure!(
            frame_index < self.frames.len(),
            "Input duration must not exceed signal duration"
        );

        if frame_index == self.frames.len() - 1 {
            if let [.., last_frame] = self.frames.as_slice() {
                return Ok(*last_frame);
            }
        }

        Ok(dsp::interpolation::linear(
            &self.frames[frame_index],
            &self.frames[frame_index + 1],
            rem,
        ))
    }
}

impl StereoSignal {
    pub fn from_path<P: AsRef<Path>>(path: P) -> eyre::Result<Self> {
        let audio_data = load_samples_from_file(path)?;

        let samples: &mut dyn Iterator<Item = f32> = match audio_data.channel_count {
            1 => &mut audio_data.samples.into_iter().zip_self(2),
            2 => &mut audio_data.samples.into_iter(),
            _ => bail!("Audio file must be mono or stereo"),
        };

        unsafe { Self::from_samples(samples.collect_vec(), audio_data.sample_rate) }
    }

    pub fn plot<P: AsRef<Path>>(&self, path: P) -> eyre::Result<()> {
        use plotters::prelude::*;

        let root = SVGBackend::new(&path, (1000, 1000)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root).build_cartesian_2d(0.0f32..0.01, -1.0f32..1.0)?;
        chart.configure_mesh().draw()?;

        let left_data = self
            .frames
            .iter()
            .enumerate()
            .map(|(i, Vector([left, _]))| (i as f32 / self.sample_rate, *left))
            .collect_vec();

        chart.draw_series(LineSeries::new(left_data, &RED))?;

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}

impl<const FRAME_SIZE: usize> Deref for Signal<FRAME_SIZE> {
    type Target = [Frame<FRAME_SIZE>];

    fn deref(&self) -> &Self::Target {
        &self.frames
    }
}

impl<const FRAME_SIZE: usize> DerefMut for Signal<FRAME_SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frames
    }
}

#[cfg(test)]
mod test {
    use crate::audio::frame::StereoFrame;

    use super::*;

    #[test]
    fn test_unsafe_into_samples_returns_correct_vec() {
        let signal = StereoSignal::from_path("assets/stereo.wav").unwrap();
        let samples = unsafe { signal.clone().into_samples() };

        assert_eq!(signal.frames.len() * 2, samples.len());

        for i in 0..signal.len() {
            let into_samples_frame: StereoFrame = [samples[2 * i], samples[2 * i + 1]].into();
            assert_eq!(signal.frames[i], into_samples_frame);
        }
    }

    #[test]
    fn test_unsafe_into_samples_with_empty_signal() {
        let signal = Signal::<1>::new(Duration::ZERO, 0.0);
        let samples = unsafe { signal.into_samples() };

        assert!(samples.is_empty());
        assert_eq!(0, samples.capacity());
    }

    #[test]
    fn test_unsafe_from_samples_stereo_yields_valid_signal() {
        let audio_data = load_samples_from_file("assets/stereo.wav").unwrap();
        let signal = unsafe {
            Signal::<2>::from_samples(audio_data.samples.clone(), audio_data.sample_rate).unwrap()
        };
        assert_eq!(audio_data.samples.len() / 2, signal.frames.len());
        for i in 0..audio_data.samples.len() / audio_data.channel_count as usize {
            let expected_frame: StereoFrame =
                [audio_data.samples[2 * i], audio_data.samples[2 * i + 1]].into();
            assert_eq!(expected_frame, signal.frames[i]);
        }
    }

    #[test]
    fn test_unsafe_from_samples_empty_yields_valid_signal() {
        let signal = unsafe { Signal::<2>::from_samples(Vec::new(), 0.0).unwrap() };
        assert!(signal.frames.is_empty());
        assert_eq!(0, signal.frames.capacity());
    }
}
