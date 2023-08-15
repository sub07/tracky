use std::time::Duration;

use rust_utils::define_value_object;

use self::pcm_sample::PcmStereoSample;

pub mod pcm_sample;
pub mod pcm_sample_player;

define_value_object!(pub Volume, f32, 1.0, |v| { (0.0..=1.0).contains(&v) });
define_value_object!(pub Pan, f32, 0.0, |v| { (-1.0..=1.0).contains(&v) });

pub fn resample(src: &PcmStereoSample, target_sample_rate: f32) -> PcmStereoSample {
    if src.sample_rate == target_sample_rate {
        return src.clone();
    }

    let src_duration = src.duration();

    let target_nb_sample = (src_duration.as_secs_f32() * target_sample_rate.round()) as usize;

    let mut time = Duration::ZERO;
    let period = Duration::from_secs_f32(1.0 / target_sample_rate);
    let mut frames = Vec::with_capacity(target_nb_sample);

    while time < src_duration {
        frames.push(src.interpolate_frame_at_time(time));
        time += period;
    }

    PcmStereoSample {
        sample_rate: target_sample_rate,
        frames,
    }
}
