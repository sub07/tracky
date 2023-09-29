use std::time::Duration;

use iter_tools::Itertools;

use crate::model::{value_object::OctaveValue, Note};

use self::{signal::StereoSignal, midi::{value_object::MidiNumber, IntoMidiNumber}};

pub mod audio_channel;
pub mod frame;
pub mod generation;
pub mod player;
pub mod signal;
pub mod midi;

pub mod value_object {
    use rust_utils::define_value_object;

    define_value_object!(pub Volume, f32, 1.0, |v| { (0.0..=1.0).contains(&v) });
    define_value_object!(pub Pan, f32, 0.0, |v| { (-1.0..=1.0).contains(&v) });
}

pub fn resample(src: &StereoSignal, target_sample_rate: f32) -> StereoSignal {
    if src.sample_rate == target_sample_rate {
        return src.clone();
    }

    let src_duration = src.duration();

    let target_nb_sample = (src_duration.as_secs_f32() * target_sample_rate.round()) as usize;

    let mut duration = Duration::ZERO;
    let period = Duration::from_secs_f32(1.0 / target_sample_rate);
    let mut frames = Vec::with_capacity(target_nb_sample);

    while duration < src_duration {
        frames.push(src.frames_at_duration(duration).unwrap());
        duration += period;
    }

    StereoSignal {
        sample_rate: target_sample_rate,
        frames,
    }
}

pub trait FrameIterator {
    fn next(
        &mut self,
        freq: f32,
        amp: f32,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Option<(f32, f32)>;

    fn collect_for_duration(
        &mut self,
        duration: Duration,
        freq: f32,
        amp: f32,
        phase: &mut f32,
        sample_rate: f32,
    ) -> StereoSignal {
        let nb_sample = sample_rate * duration.as_secs_f32();
        let frames = (0..nb_sample as usize)
            .map_while(|_| self.next(freq, amp, phase, sample_rate))
            .collect_vec();
        StereoSignal::from_frames(frames, sample_rate)
    }
}

pub trait IntoFrequency {
    fn into_frequency(self) -> f32;
}

impl IntoFrequency for MidiNumber {
    // f(midi_num) = 440 * 2^((n - 69) / 12)
    // a = 440
    // b = 2^((n - 69) / 12)
    fn into_frequency(self) -> f32 {
        let midi_value = self.value() as f64;
        let a = 440.0;

        let b_pow = (midi_value - 69.0) / 12.0;
        let b = 2.0f64.powf(b_pow);

        (a * b) as f32
    }
}

impl <T: IntoMidiNumber> IntoFrequency for T {
    fn into_frequency(self) -> f32 {
        self.into_midi_note().into_frequency()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn A4_should_be_freq_440_0() {
        let freq = (Note::A, OctaveValue::new(4).unwrap()).into_frequency();
        assert_eq!(440.0, freq);
    }

    #[test]
    fn B2_should_be_freq_123_47() {
        let freq = (Note::B, OctaveValue::new(2).unwrap()).into_frequency();
        approx::assert_relative_eq!(123.47, freq, epsilon = 0.001);
    }
}