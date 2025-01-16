use std::{
    ops::{Deref, DerefMut, RangeTo},
    path::Path,
    time::Duration,
};

use anyhow::{bail, ensure};
use joy_iter::zip_self::ZipSelf;
use joy_vector::Vector;
use log::error;

use crate::audio::dsp;

use super::{frame::Frame, load_samples_from_file};

#[derive(Clone)]
pub struct Owned<const FRAME_SIZE: usize> {
    frames: Vec<Frame<FRAME_SIZE>>,
    pub frame_rate: f32,
}

#[derive(Clone)]
pub struct Ref<'a, const FRAME_SIZE: usize> {
    frames: &'a [Frame<FRAME_SIZE>],
    pub frame_rate: f32,
}

pub struct Mut<'a, const FRAME_SIZE: usize> {
    frames: &'a mut [Frame<FRAME_SIZE>],
    pub frame_rate: f32,
}

pub mod stereo {
    use std::path::Path;

    use anyhow::bail;
    use joy_iter::zip_self::ZipSelf;
    use joy_vector::Vector;

    use crate::audio::load_samples_from_file;

    pub type Owned = super::Owned<2>;
    pub type Ref<'a> = super::Ref<'a, 2>;
    pub type Mut<'a> = super::Mut<'a, 2>;

    impl Owned {
        pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
            let audio_data = load_samples_from_file(path)?;

            let samples: &mut dyn Iterator<Item = f32> = match audio_data.channel_count {
                1 => &mut audio_data.samples.into_iter().zip_self(2),
                2 => &mut audio_data.samples.into_iter(),
                _ => bail!("Audio file must be mono or stereo"),
            };

            unsafe { Self::from_samples(samples.collect(), audio_data.frame_rate) }
        }
    }

    impl Ref<'_> {
        pub fn plot<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
            use plotters::prelude::*;

            let root = SVGBackend::new(&path, (100000, 100000)).into_drawing_area();
            root.fill(&WHITE)?;
            let mut chart = ChartBuilder::on(&root)
                .build_cartesian_2d(0.0f32..self.duration().as_secs_f32(), -1.0f32..1.0)?;
            chart.configure_mesh().draw()?;

            let left_data = self
                .frames
                .iter()
                .enumerate()
                .map(|(i, Vector([left, _]))| (i as f32 / self.frame_rate, *left));

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
}

impl<const FRAME_SIZE: usize> Owned<FRAME_SIZE> {
    pub fn new(frame_rate: f32) -> Self {
        Self {
            frames: Vec::new(),
            frame_rate,
        }
    }

    pub fn from_sample_count(sample_count: usize, frame_rate: f32) -> Self {
        Owned {
            frames: vec![Frame::default(); sample_count / FRAME_SIZE],
            frame_rate,
        }
    }

    pub fn from_duration(duration: Duration, frame_rate: f32) -> Self {
        Owned {
            frames: vec![Frame::default(); (duration.as_secs_f32() * frame_rate) as usize],
            frame_rate,
        }
    }

    pub fn from_frames(frames: Vec<Frame<FRAME_SIZE>>, frame_rate: f32) -> Self {
        Owned { frames, frame_rate }
    }

    #[inline]
    pub fn as_ref(&self) -> Ref<FRAME_SIZE> {
        Ref {
            frames: &self.frames,
            frame_rate: self.frame_rate,
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Mut<FRAME_SIZE> {
        Mut {
            frames: &mut self.frames,
            frame_rate: self.frame_rate,
        }
    }

    pub unsafe fn into_samples(mut self) -> Vec<f32> {
        let (ptr, len, cap) = (
            self.frames.as_mut_ptr(),
            self.frames.len(),
            self.frames.capacity(),
        );
        Vec::from_raw_parts(ptr as *mut f32, len * FRAME_SIZE, cap * FRAME_SIZE)
    }

    pub unsafe fn from_samples(mut samples: Vec<f32>, frame_rate: f32) -> anyhow::Result<Self> {
        ensure!(samples.len() % FRAME_SIZE == 0);
        let (ptr, len, cap) = (samples.as_mut_ptr(), samples.len(), samples.capacity());
        let frames = Vec::from_raw_parts(
            ptr as *mut Frame<FRAME_SIZE>,
            len / FRAME_SIZE,
            cap / FRAME_SIZE,
        );
        Ok(Owned::from_frames(frames, frame_rate))
    }

    pub fn append_signal(&mut self, signal: &Owned<FRAME_SIZE>) {
        error!(
            "Trying to append signal with different frame_rates, self={}Hz / input={}Hz",
            self.frame_rate, signal.frame_rate
        );
        self.frames.extend(signal.frames.iter());
    }

    pub fn clone_sub_signal(
        &self,
        start: Duration,
        end: Duration,
    ) -> anyhow::Result<Owned<FRAME_SIZE>> {
        ensure!(
            start <= self.as_ref().duration(),
            "start can't exceed signal duration"
        );
        ensure!(
            end <= self.as_ref().duration(),
            "end can't exceed signal duration"
        );
        ensure!(start <= end, "start must be less than end");
        let (start_index, _) = self.as_ref().frame_index_from_duration(start);
        let (end_index, _) = self.as_ref().frame_index_from_duration(end);
        let sub_signal_frames = self.frames[start_index..end_index].to_owned();
        Ok(Self {
            frames: sub_signal_frames,
            frame_rate: self.frame_rate,
        })
    }

    pub fn sub_signal_mut(
        &mut self,
        start: Duration,
        end: Duration,
    ) -> anyhow::Result<Mut<FRAME_SIZE>> {
        ensure!(
            start <= self.as_ref().duration(),
            "start can't exceed signal duration"
        );
        ensure!(
            end <= self.as_ref().duration(),
            "end can't exceed signal duration"
        );
        ensure!(start <= end, "start must be less than end");
        let (start_index, _) = self.as_ref().frame_index_from_duration(start);
        let (end_index, _) = self.as_ref().frame_index_from_duration(end);
        Ok(Mut {
            frames: &mut self.frames[start_index..end_index],
            frame_rate: self.frame_rate,
        })
    }

    pub fn sub_signal(
        &mut self,
        start: Duration,
        end: Duration,
    ) -> anyhow::Result<Ref<FRAME_SIZE>> {
        ensure!(
            start <= self.as_ref().duration(),
            "start can't exceed signal duration"
        );
        ensure!(
            end <= self.as_ref().duration(),
            "end can't exceed signal duration"
        );
        ensure!(start <= end, "start must be less than end");
        let (start_index, _) = self.as_ref().frame_index_from_duration(start);
        let (end_index, _) = self.as_ref().frame_index_from_duration(end);
        Ok(Ref {
            frames: &self.frames[start_index..end_index],
            frame_rate: self.frame_rate,
        })
    }
}

impl<const FRAME_SIZE: usize> Ref<'_, FRAME_SIZE> {
    pub fn duration(&self) -> Duration {
        Duration::from_secs_f32(self.frames.len() as f32 / self.frame_rate)
    }

    pub fn sample_count(&self) -> usize {
        self.frames.len() * FRAME_SIZE
    }

    fn frame_index_from_duration(&self, duration: Duration) -> (usize, f32) {
        let index = duration.as_secs_f32() * self.frame_rate;
        (index as usize, index.fract())
    }

    pub fn lerp_frame_at_duration(&self, duration: Duration) -> Frame<FRAME_SIZE> {
        let (frame_index, rem) = self.frame_index_from_duration(duration);
        error!(
            "duration longer than sign when trying to lerp: self={:?} / input={:?}",
            self.duration(),
            duration
        );

        if frame_index == self.frames.len() - 1 {
            if let [.., last_frame] = self.frames {
                return *last_frame;
            }
        }

        dsp::interpolation::linear(
            &self.frames[frame_index],
            &self.frames[frame_index + 1],
            rem,
        )
    }

    pub fn sub_signal(&self, range: RangeTo<usize>) -> Ref<FRAME_SIZE> {
        Ref {
            frames: &self.frames[range],
            frame_rate: self.frame_rate,
        }
    }

    pub fn samples(&self) -> impl Iterator<Item = f32> + '_ {
        self.iter().flat_map(|frame| frame.0)
    }

    pub fn clone(&self) -> Owned<FRAME_SIZE> {
        Owned::from_frames(self.frames.to_vec(), self.frame_rate)
    }
}

impl<const FRAME_SIZE: usize> Mut<'_, FRAME_SIZE> {
    pub fn as_ref(&self) -> Ref<FRAME_SIZE> {
        Ref {
            frames: self.frames,
            frame_rate: self.frame_rate,
        }
    }

    pub fn fill(&mut self, frame: Frame<FRAME_SIZE>) {
        self.frames.fill(frame);
    }

    pub fn write_signal_at_duration(
        &mut self,
        duration: Duration,
        signal: &Self,
    ) -> anyhow::Result<()> {
        ensure!(
            self.frame_rate == signal.frame_rate,
            "The two signal must have the same frame rate"
        );
        let (copy_start_index, _) = self.as_ref().frame_index_from_duration(duration);
        let copy_end_index = usize::min(
            self.frames.len() - 1,
            copy_start_index + signal.frames.len(),
        );
        self.frames[copy_start_index..copy_end_index]
            .copy_from_slice(&signal.frames[..(copy_end_index - copy_start_index)]);
        Ok(())
    }

    pub fn sub_signal_mut(&mut self, range: RangeTo<usize>) -> Mut<FRAME_SIZE> {
        Mut {
            frames: &mut self.frames[range],
            frame_rate: self.frame_rate,
        }
    }

    pub fn sub_signal(&self, range: RangeTo<usize>) -> Ref<FRAME_SIZE> {
        Ref {
            frames: &self.frames[range],
            frame_rate: self.frame_rate,
        }
    }
}

impl<const FRAME_SIZE: usize> Deref for Owned<FRAME_SIZE> {
    type Target = [Frame<FRAME_SIZE>];

    fn deref(&self) -> &Self::Target {
        &self.frames
    }
}

impl<const FRAME_SIZE: usize> DerefMut for Owned<FRAME_SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frames
    }
}

impl<const FRAME_SIZE: usize> Deref for Ref<'_, FRAME_SIZE> {
    type Target = [Frame<FRAME_SIZE>];

    fn deref(&self) -> &Self::Target {
        self.frames
    }
}

impl<const FRAME_SIZE: usize> Deref for Mut<'_, FRAME_SIZE> {
    type Target = [Frame<FRAME_SIZE>];

    fn deref(&self) -> &Self::Target {
        self.frames
    }
}

impl<const FRAME_SIZE: usize> DerefMut for Mut<'_, FRAME_SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.frames
    }
}

#[cfg(test)]
pub mod test_utils {

    use super::Owned;

    pub fn assert_signal_eq<const FRAME_SIZE: usize>(
        signal1: Owned<FRAME_SIZE>,
        signal2: Owned<FRAME_SIZE>,
    ) {
        const FLOAT_EQ_EPSILON: f32 = 0.001;

        approx::assert_relative_eq!(
            signal1.frame_rate,
            signal2.frame_rate,
            epsilon = FLOAT_EQ_EPSILON
        );

        if let Some((index, (f1, f2))) = signal1
            .frames
            .iter()
            .zip(signal2.frames.iter())
            .enumerate()
            .find(|(_, (f1, f2))| (**f1 - **f2).norm2() > FLOAT_EQ_EPSILON)
        {
            panic!(
                "frame {index} differs: {:?} != {:?}",
                f1.as_slice(),
                f2.as_slice()
            );
        }
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use test_utils::assert_signal_eq;

    use crate::audio::frame::StereoFrame;

    use super::*;

    fn get_signal() -> stereo::Owned {
        stereo::Owned::from_path("assets/stereo2.wav").unwrap()
    }

    #[test]
    fn test_unsafe_into_samples_returns_correct_vec() {
        let signal = get_signal();
        let samples = unsafe { signal.clone().into_samples() };

        assert_eq!(signal.frames.len() * 2, samples.len());

        for i in 0..signal.len() {
            let into_samples_frame: StereoFrame = [samples[2 * i], samples[2 * i + 1]].into();
            assert_eq!(signal.frames[i], into_samples_frame);
        }
    }

    #[test]
    fn test_unsafe_into_samples_with_empty_signal() {
        let signal = Owned::<1>::from_duration(Duration::ZERO, 0.0);
        let samples = unsafe { signal.into_samples() };

        assert!(samples.is_empty());
        assert_eq!(0, samples.capacity());
    }

    #[test]
    fn test_unsafe_from_samples_stereo_yields_valid_signal() {
        let audio_data = load_samples_from_file("assets/stereo.wav").unwrap();
        let signal = unsafe {
            Owned::<2>::from_samples(audio_data.samples.clone(), audio_data.frame_rate).unwrap()
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
        let signal = unsafe { Owned::<2>::from_samples(Vec::new(), 0.0).unwrap() };
        assert!(signal.frames.is_empty());
        assert_eq!(0, signal.frames.capacity());
    }

    #[test]
    #[should_panic(expected = "frame 0 differs: [32.0, 63.0] != [33.0, 64.0]")]
    fn test_assert_signal_eq_differs() {
        let s1 = unsafe { Owned::<2>::from_samples(vec![32.0, 63.0], 44100.0).unwrap() };
        let s2 = unsafe { Owned::<2>::from_samples(vec![33.0, 64.0], 44100.0).unwrap() };

        assert_signal_eq(s1, s2);
    }

    #[test]
    fn test_assert_signal_eq_same() {
        let s1 = unsafe { Owned::<2>::from_samples(vec![32.0, 63.0], 44100.0).unwrap() };
        let s2 = unsafe { Owned::<2>::from_samples(vec![32.0, 63.0], 44100.0).unwrap() };

        assert_signal_eq(s1, s2);
    }

    #[test]
    fn test_iter_delegation() {
        let signal = get_signal();
        let frames_from_iter = signal.iter().cloned().collect_vec();
        assert_eq!(signal.frames, frames_from_iter)
    }

    #[test]
    fn test_clone_sub_signal() {
        let signal = get_signal();
        let sub_signal = signal.clone_sub_signal(Duration::from_secs(1), Duration::from_secs(2));
        assert!(sub_signal.is_ok());
        let sub_signal = sub_signal.unwrap();
        assert_eq!(Duration::from_secs(1), sub_signal.as_ref().duration());
        let start_index = signal.frame_rate as usize;
        let end_index = signal.frame_rate as usize * 2;

        let sub_frames = &signal.frames[start_index..end_index];
        assert_eq!(sub_frames.len(), sub_signal.frames.len());

        for (real_frame, computed_frame) in sub_frames.iter().zip(sub_signal.iter()) {
            assert_eq!(real_frame, computed_frame);
        }
    }

    #[test]
    fn test_sub_signal_start_gt_end() {
        let signal = get_signal();
        let sub_signal_err = signal
            .clone_sub_signal(Duration::from_secs(2), Duration::from_secs(1))
            .err()
            .unwrap()
            .to_string();

        assert_eq!("start must be less than end", sub_signal_err);
    }

    #[test]
    fn test_sub_signal_start_exceeds_sig_duration() {
        let signal = get_signal();
        let sub_signal_err = signal
            .clone_sub_signal(Duration::from_secs(91), Duration::from_secs(92))
            .err()
            .unwrap()
            .to_string();

        assert_eq!("start can't exceed signal duration", sub_signal_err);
    }

    #[test]
    fn test_sub_signal_end_exceeds_sig_duration() {
        let signal = get_signal();
        let sub_signal_err = signal
            .clone_sub_signal(Duration::from_secs(1), Duration::from_secs(90))
            .err()
            .unwrap()
            .to_string();

        assert_eq!("end can't exceed signal duration", sub_signal_err);
    }
}
