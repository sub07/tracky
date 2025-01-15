use std::time::Duration;

use log::warn;

use super::{
    frame::Frame,
    signal::{Signal, StereoSignal},
};

#[derive(Clone)]
pub struct Mixer {
    pub signal: StereoSignal,
}

impl std::ops::Deref for Mixer {
    type Target = StereoSignal;

    fn deref(&self) -> &Self::Target {
        &self.signal
    }
}

impl Mixer {
    pub fn from_sample_buffer_size(sample_buffer_size: usize, frame_rate: f32) -> Mixer {
        Mixer {
            signal: Signal::from_sample_buffer_size(sample_buffer_size, frame_rate),
        }
    }

    pub fn mix(&mut self, signal: &StereoSignal) {
        debug_assert_eq!(self.signal.frame_rate, signal.frame_rate);
        if signal.len() != self.signal.len() {
            warn!("Attempt to mix two signal of different size, mixer signal len: {} / input signal len: {}. Truncation may happen", self.signal.len(), signal.len());
        }
        for (mut output, input) in self.signal.frames.iter_mut().zip(signal.frames.iter()) {
            output += input;
        }
    }

    pub fn reset(&mut self) {
        self.signal.fill(Frame::default());
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::audio::signal::test_utils::assert_signal_eq;

    use super::*;
    const TEST_SAMPLE_RATE: f32 = 44100.0;

    fn get_mixer() -> Mixer {
        Mixer::from_sample_buffer_size(0, TEST_SAMPLE_RATE)
    }

    fn get_short_signal() -> StereoSignal {
        StereoSignal::from_path("assets/stereo.wav").unwrap()
    }

    fn get_long_signal() -> StereoSignal {
        StereoSignal::from_path("assets/stereo2.wav").unwrap()
    }

    #[test]
    fn test_empty_mixer() {
        let mixer = get_mixer();

        assert!(mixer.signal.is_empty())
    }

    #[test]
    fn test_one_mix() {
        let mut mixer = get_mixer();
        let signal = get_short_signal();

        mixer.mix(&signal);

        assert_signal_eq(mixer.signal, signal);
    }

    #[test]
    fn test_two_same_mix() {
        let mut mixer = get_mixer();
        let mut signal = get_short_signal();

        mixer.mix(&signal);
        mixer.mix(&signal);

        for mut frame in signal.frames.iter_mut() {
            frame *= 2.0;
        }

        assert_signal_eq(mixer.signal, signal);
    }

    #[test]
    fn test_three_same_mix() {
        let mut mixer = get_mixer();
        let mut signal = get_short_signal();

        mixer.mix(&signal);
        mixer.mix(&signal);
        mixer.mix(&signal);

        for mut frame in signal.frames.iter_mut() {
            frame *= 3.0;
        }

        assert_signal_eq(mixer.signal, signal);
    }

    #[test]
    fn test_mix_different_signal() {
        let mut mixer = get_mixer();
        let s1 = get_short_signal();
        let s2 = get_long_signal();

        mixer.mix(&s1);
        mixer.mix(&s2);

        assert_eq!(s2.frames.len(), mixer.signal.len());
        let first_part = Signal::from_frames(
            s1.iter()
                .zip(s2.iter())
                .map(|(f1, f2)| f1 + f2)
                .collect_vec(),
            s1.frame_rate,
        );
        let second_part = s2.sub_signal(s1.duration(), s2.duration()).unwrap();

        assert_signal_eq(
            first_part,
            mixer
                .signal
                .sub_signal(Duration::ZERO, s1.duration())
                .unwrap(),
        );
        assert_signal_eq(
            second_part,
            mixer
                .signal
                .sub_signal(s1.duration(), s2.duration())
                .unwrap(),
        );
    }
}
