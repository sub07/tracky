use std::time::Duration;
use crate::audio::sound::Sound;
use crate::define_value_object;

pub mod sound;
pub mod stream;
pub mod channel;

define_value_object!(pub Volume, f32, 1.0, |v| { (0.0..=1.0).contains(&v) });
define_value_object!(pub Pan, f32, 0.0, |v| { (-1.0..=1.0).contains(&v) });
define_value_object!(pub SampleRate, f32, 44100.0);

fn resample(src: &Sound, target: SampleRate) -> Sound {
    if src.sample_rate == target { return src.clone(); }

    let src_duration = src.duration();
    let target_sr = target.value();

    let target_nb_sample = (src_duration.as_secs_f32() * target_sr.round()) as usize;

    let mut time = Duration::ZERO;
    let period = Duration::from_secs_f32(1.0 / target_sr);
    let mut frames = Vec::with_capacity(target_nb_sample);

    while time < src_duration {
        frames.push(src.interpolate_frame_at_time(time));
        time += period;
    }

    Sound {
        sample_rate: target,
        frames,
    }
}
