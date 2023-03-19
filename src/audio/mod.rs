use std::time::Duration;
use crate::audio::sound::Sound;
use crate::audio::value_object::SampleRate;

pub mod sound;
pub mod stream;
mod value_object;

fn resample(src: &Sound, target: SampleRate) -> Sound {
    if src.sample_rate == target {
        return Sound {
            samples: src.samples.clone(),
            sample_rate: target,
        };
    }


    let src_duration = src.duration();
    let target_sr = target.value();

    let target_nb_sample = ((src_duration.as_secs_f32() * target_sr.round()) as usize) * src.nb_channel();

    let mut time = Duration::ZERO;
    let period = Duration::from_secs_f32(1.0 / target_sr);
    let mut samples = Vec::with_capacity(target_nb_sample);

    while time < src_duration {
        let (l, r) = src.interpolate_at_time(time);
        samples.push(l);
        samples.push(r);
        time += period;
    }

    Sound {
        sample_rate: target,
        samples,
    }
}
