use std::time::Duration;
use crate::audio::sound::Sound;

pub mod sound;
pub mod stream;

fn resample(src: &Sound, target_sr: f64) -> Sound {
    if src.speed == target_sr {
        return Sound {
            samples: src.samples.clone(),
            speed: target_sr,
        };
    }

    let src_duration = src.duration();

    let target_nb_sample = ((src_duration.as_secs_f64() * target_sr.round()) as usize) * src.nb_channel();

    let mut time = Duration::ZERO;
    let period = Duration::from_secs_f64(1.0 / target_sr);
    let mut samples = Vec::with_capacity(target_nb_sample);

    while time < src_duration {
        let (l, r) = src.sample_at_time(time);
        samples.push(l);
        samples.push(r);
        time += period;
    }

    Sound {
        speed: target_sr,
        samples,
    }
}
