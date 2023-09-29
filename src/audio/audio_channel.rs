use std::time::Duration;

use crate::model::pattern::Column;

use super::{
    generation::{
        SampleParametersInterpolator, SawWaveDescriptor, SineWaveDescriptor, SquareWaveDescriptor,
    },
    signal::StereoSignal,
    Samples,
};

pub fn handle_column(bps: f64, out: &mut StereoSignal, column: &Column) {
    let mut current_instrument: Option<(Box<dyn Samples>, u8)> = None;
    let step_duration = Duration::from_secs_f64(1.0 / bps);
    let mut current_duration = Duration::ZERO;
    let mut phase = 0.0;
    let mut current_velocity = 1.0;

    for line in &column.lines {
        if let Some(new_instrument_index) = line.instrument_field.value {
            let instrument = match new_instrument_index {
                0 => Some(Box::new(SampleParametersInterpolator::new(SineWaveDescriptor)) as _),
                1 => Some(Box::new(SampleParametersInterpolator::new(SquareWaveDescriptor)) as _),
                2 => Some(Box::new(SampleParametersInterpolator::new(SawWaveDescriptor)) as _),
                _ => None,
            };
            if let Some((_, current_instrument_index)) = current_instrument {
                if current_instrument_index != new_instrument_index {
                    phase = 0.0;
                }
            } else {
                phase = 0.0;
            }
            current_instrument = instrument.map(|i| (i, new_instrument_index));
        }
        if let Some(velocity) = line.velocity_field.value {
            current_velocity = velocity as f32 / u8::MAX as f32;
        }
        if let Some((instrument, _)) = current_instrument.as_mut() {
            let pcm_frames = instrument.collect_for_duration(
                step_duration,
                440.0,
                current_velocity,
                &mut phase,
                out.sample_rate,
            );
            out.write_frames_at_duration(current_duration, &pcm_frames);
        }
        current_duration += step_duration;
    }
}
