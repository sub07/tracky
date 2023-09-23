use std::time::Duration;

use iter_tools::Itertools;
use rust_utils::define_value_object;

use self::signal::StereoSignal;

pub mod audio_channel;
pub mod frame;
pub mod generation;
pub mod pcm_sample_player;
pub mod signal;

define_value_object!(pub Volume, f32, 1.0, |v| { (0.0..=1.0).contains(&v) });
define_value_object!(pub Pan, f32, 0.0, |v| { (-1.0..=1.0).contains(&v) });

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

pub trait Samples {
    fn next(&mut self, freq: f32, amp: f32, phase: &mut f32, sample_rate: f32) -> Option<(f32, f32)>;

    fn collect_for_duration(
        &mut self,
        duration: Duration,
        freq: f32,
        amp: f32,
        phase: &mut f32,
        sample_rate: f32,
    ) -> Vec<(f32, f32)> {
        let nb_sample = sample_rate * duration.as_secs_f32();
        (0..nb_sample as usize)
            .map_while(|_| self.next(freq, amp, phase, sample_rate))
            .collect_vec()
    }
}
