use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use frame::Frame;

pub mod frame;

#[derive(Clone, PartialEq)]
pub struct Signal<const FRAME_SIZE: usize> {
    pub frames: Vec<Frame<FRAME_SIZE>>,
    pub frame_rate: f32,
}

impl<const FRAME_SIZE: usize> Signal<FRAME_SIZE> {
    pub fn new(duration: Duration, frame_rate: f32) -> Self {
        Signal {
            frames: vec![Frame::default(); (duration.as_secs_f32() * frame_rate) as usize],
            frame_rate,
        }
    }

    pub fn from_frames(frames: Vec<Frame<FRAME_SIZE>>, frame_rate: f32) -> Self {
        Signal { frames, frame_rate }
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs_f32(self.frames.len() as f32 / self.frame_rate)
    }

    pub fn samples(&self) -> impl Iterator<Item = f32> + '_ {
        self.frames.iter().flat_map(|frame| frame.iter()).cloned()
    }

    fn frame_index_from_duration(&self, duration: Duration) -> (usize, f32) {
        let index = duration.as_secs_f32() * self.frame_rate;
        (index as usize, index.fract())
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
pub mod test_utils {

    use super::Signal;

    // In strict mode signal length must be the same
    // In non-strict mode signal length can differ and the longer signal will be trimmed down to the shorter before comparison
    pub fn assert_signal_eq<const FRAME_SIZE: usize>(
        expected: Signal<FRAME_SIZE>,
        actual: Signal<FRAME_SIZE>,
        strict: bool,
    ) {
        const FLOAT_EQ_EPSILON: f32 = 0.001;

        approx::assert_relative_eq!(
            expected.frame_rate,
            actual.frame_rate,
            epsilon = FLOAT_EQ_EPSILON
        );

        if strict {
            assert_eq!(expected.frames.len(), actual.frames.len());
        }

        if let Some((index, (expected_frame, actual_frame))) = expected
            .frames
            .iter()
            .zip(actual.frames.iter())
            .enumerate()
            .find(|(_, (f1, f2))| (**f1 - **f2).norm2() > FLOAT_EQ_EPSILON)
        {
            panic!(
                "frame #{index} differs: \n\tExpected: {:?}\n\tGot: {:?}",
                expected_frame.as_slice(),
                actual_frame.as_slice()
            );
        }
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use test_utils::assert_signal_eq;

    use super::*;

    #[test]
    fn test_new() {
        const DURATION: Duration = Duration::from_secs(4);
        const FRAME_RATE: f32 = 2.0;
        const FRAME_SIZE: usize = 2;
        let expected = Signal {
            frames: vec![Frame::<FRAME_SIZE>::default(); 8],
            frame_rate: FRAME_RATE,
        };
        let actual = Signal::<FRAME_SIZE>::new(DURATION, FRAME_RATE);
        assert_signal_eq(expected, actual, true);
    }

    #[test]
    fn test_from_frames() {
        const FRAME_RATE: f32 = 2.0;
        let frames = vec![
            [1.2, 4.3].into(),
            [4.2, 5.6].into(),
            [7.2, 5.6].into(),
            [7.9, 4.1].into(),
        ];
        let expected = Signal {
            frames: frames.clone(),
            frame_rate: FRAME_RATE,
        };
        let actual = Signal::from_frames(frames, FRAME_RATE);
        assert_signal_eq(expected, actual, true);
    }

    #[test]
    #[should_panic(
        expected = "cannot convert float seconds to Duration: value is either too big or NaN"
    )]
    fn test_duration_invalid_frame_rate() {
        const DURATION: Duration = Duration::from_secs(4);
        let expected = DURATION;
        let actual = Signal::<2>::new(DURATION, 0.0).duration();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_duration_valid_frame_rate() {
        const DURATION: Duration = Duration::from_secs(4);
        let expected = DURATION;
        let actual = Signal::<2>::new(DURATION, 2.0).duration();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_frame_index_from_duration() {
        fn assert_frame_index_eq(
            (expected_index, expected_rem): (usize, f32),
            (actual_index, actual_rem): (usize, f32),
        ) {
            assert_eq!(expected_index, actual_index);
            approx::assert_relative_eq!(expected_rem, actual_rem, epsilon = 0.001);
        }

        const FRAME_RATE: f32 = 2.0;
        let s = Signal::<2>::new(Duration::from_secs(10), FRAME_RATE);

        assert_frame_index_eq((0, 0.0), s.frame_index_from_duration(Duration::ZERO));
        assert_frame_index_eq(
            (20, 0.0),
            s.frame_index_from_duration(Duration::from_secs(10)),
        );
        assert_frame_index_eq(
            (10, 0.0),
            s.frame_index_from_duration(Duration::from_secs(5)),
        );
        assert_frame_index_eq(
            (40, 0.0),
            s.frame_index_from_duration(Duration::from_secs(20)),
        );
        assert_frame_index_eq(
            (5, 0.0),
            s.frame_index_from_duration(Duration::from_secs_f32(2.5)),
        );
        assert_frame_index_eq(
            (4, 0.5),
            s.frame_index_from_duration(Duration::from_secs_f32(2.25)),
        );
        assert_frame_index_eq(
            (15, 0.5),
            s.frame_index_from_duration(Duration::from_secs_f32(7.75)),
        );
        assert_frame_index_eq(
            (8, 0.53),
            s.frame_index_from_duration(Duration::from_secs_f32(4.265)),
        );
    }

    #[test]
    fn test_samples_iter_stereo() {
        let s = Signal::<2>::new(Duration::from_secs(5), 2.0);
        let samples = s.samples().collect_vec();
        assert_eq!(vec![0.0; 20], samples);
    }

    #[test]
    fn test_samples_iter_mono() {
        let s = Signal::<1>::new(Duration::from_secs(5), 2.0);
        let samples = s.samples().collect_vec();
        assert_eq!(vec![0.0; 10], samples);
    }

    #[test]
    fn test_samples_iter_values() {
        let frames = vec![
            [1.2, 4.3].into(),
            [4.2, 5.6].into(),
            [7.2, 5.6].into(),
            [7.9, 4.1].into(),
        ];
        let s = Signal::from_frames(frames, 2.0);
        let samples = s.samples().collect_vec();
        assert_eq!(vec![1.2, 4.3, 4.2, 5.6, 7.2, 5.6, 7.9, 4.1], samples);
    }
}
