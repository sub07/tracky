use std::time::Duration;

use crate::audio::signal;

#[allow(dead_code)]
pub fn linear<const FRAME_SIZE: usize>(
    src: signal::Ref<FRAME_SIZE>,
    target_sample_rate: f32,
) -> signal::Owned<FRAME_SIZE> {
    let target_frame_rate = target_sample_rate.round();

    if src.frame_rate == target_frame_rate {
        return src.clone();
    }

    let src_duration = src.duration();
    let target_frame_count = (src_duration.as_secs_f32() * target_frame_rate) as usize;

    let mut current_duration = Duration::ZERO;
    let period = Duration::from_secs_f32(1.0 / target_frame_rate);
    let mut frames = Vec::with_capacity(target_frame_count);

    while current_duration < src_duration {
        frames.push(src.lerp_frame_at_duration(current_duration).unwrap());
        current_duration += period;
    }

    signal::Owned::from_frames(frames, target_frame_rate)
}
