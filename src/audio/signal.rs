use std::{
    ops::{Deref, DerefMut},
    path::Path,
    time::Duration,
};

use eyre::ensure;
use itertools::Itertools;
use joy_iter::zip_self::ZipSelf;

use crate::audio::dsp;

use super::frame::{FrameExt, StereoFrame};

pub type StereoSignal = Signal<StereoFrame>;

#[derive(Clone)]
pub struct Signal<F> {
    pub frames: Vec<F>,
    pub sample_rate: f32,
}

impl<F> Signal<F>
where
    F: Default + Clone,
{
    pub fn new(duration: Duration, sample_rate: f32) -> Self {
        Signal {
            frames: vec![F::default(); (duration.as_secs_f32() * sample_rate) as usize],
            sample_rate,
        }
    }
}

impl<F> Signal<F>
where
    F: Clone + Copy,
{
    pub fn from_frames(frames: Vec<F>, sample_rate: f32) -> Self {
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
}

impl<F> Signal<F>
where
    F: FrameExt,
{
    pub fn into_samples(self) -> Vec<F::Sample> {
        let (ptr, len, cap) = self.frames.into_raw_parts();
        unsafe {
            Vec::from_raw_parts(
                ptr as *mut F::Sample,
                len * F::FRAME_SIZE,
                cap * F::FRAME_SIZE,
            )
        }
    }
}

impl<F> Signal<F>
where
    F: FrameExt + Clone + Copy,
{
    pub fn from_samples(samples: Vec<F::Sample>, sample_rate: f32) -> eyre::Result<Self> {
        ensure!(samples.len() % F::FRAME_SIZE == 0);
        let (ptr, len, cap) = samples.into_raw_parts();
        let frames =
            unsafe { Vec::from_raw_parts(ptr as *mut F, len / F::FRAME_SIZE, cap / F::FRAME_SIZE) };
        Ok(Signal::from_frames(frames, sample_rate))
    }
}

impl StereoSignal {
    pub fn from_path<P: AsRef<Path>>(path: P) -> eyre::Result<Self> {
        let mut audio_file = audrey::open(path)?;
        let desc = audio_file.description();

        ensure!(
            desc.channel_count() <= 2,
            "Audio file must be mono or stereo"
        );

        let mut samples = audio_file.samples().map_while(|f| f.ok());
        let samples: &mut dyn Iterator<Item = f32> = if desc.channel_count() == 1 {
            &mut samples.zip_self(2)
        } else {
            &mut samples
        };

        let samples = samples.into_iter().tuples().collect_vec();

        Ok(Self {
            frames: samples,
            sample_rate: desc.sample_rate() as f32,
        })
    }

    pub fn interpolate_frame_at_duration(&self, duration: Duration) -> eyre::Result<StereoFrame> {
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

        let f1 = self.frames[frame_index];
        let f2 = self.frames[frame_index + 1];

        dsp::interpolation::linear_f32(f1, f2, rem)
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
            .map(|(i, (l, _))| (i as f32 / self.sample_rate, *l))
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

impl<F> Deref for Signal<F> {
    type Target = [F];

    fn deref(&self) -> &Self::Target {
        &self.frames
    }
}

impl<F> DerefMut for Signal<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frames
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_that_unsafe_into_samples_returns_correct_vec() {
        let signal = StereoSignal::from_path("assets/piano.wav").unwrap();
        let samples = signal.clone().into_samples();

        for i in 0..signal.len() {
            assert_eq!((samples[2 * i], samples[2 * i + 1]), signal.frames[i]);
        }
    }
}
