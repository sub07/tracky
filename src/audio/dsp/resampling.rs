use std::time::Duration;

use crate::audio::signal::Signal;

#[allow(dead_code)]
pub fn linear<const FRAME_SIZE: usize>(
    src: &Signal<FRAME_SIZE>,
    target_sample_rate: f32,
) -> Signal<FRAME_SIZE> {
    let target_sample_rate = target_sample_rate.round();

    if src.frame_rate == target_sample_rate {
        return src.clone();
    }

    let src_duration = src.duration();
    let target_frame_count = (src_duration.as_secs_f32() * target_sample_rate) as usize;

    let mut current_duration = Duration::ZERO;
    let period = Duration::from_secs_f32(1.0 / target_sample_rate);
    let mut frames = Vec::with_capacity(target_frame_count);

    while current_duration < src_duration {
        frames.push(
            src.lerp_frame_at_duration(current_duration)
                .expect("Algo is wrong"),
        );
        current_duration += period;
    }

    Signal {
        frames,
        frame_rate: target_sample_rate,
    }
}
