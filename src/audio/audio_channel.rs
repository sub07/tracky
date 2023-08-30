use std::time::Duration;

use crate::model::pattern::Column;

use super::{
    generation::{SawWaveDescriptor, SineWaveDescriptor, SquareWaveDescriptor},
    pcm_sample::PcmStereoSample,
    Samples,
};

pub fn handle_column(bps: f64, out: &mut PcmStereoSample, column: &Column) {
    let mut current_instrument: Option<Box<dyn Samples>> = None;
    let step_duration = Duration::from_secs_f64(1.0 / bps);
    let mut current_duration = Duration::ZERO;

    for line in &column.lines {
        if let Some(instrument_index) = line.instrument_field.value {
            let instrument = match instrument_index {
                0 => Some(Box::new(SineWaveDescriptor::new(1.0)) as Box<dyn Samples>),
                1 => Some(Box::new(SquareWaveDescriptor::new(1.0)) as Box<dyn Samples>),
                2 => Some(Box::new(SawWaveDescriptor::new(1.0)) as Box<dyn Samples>),
                _ => None,
            };
            current_instrument = instrument;
        }
        if let Some(instrument) = current_instrument.as_mut() {
            let pcm_frames = instrument.collect_for_duration(step_duration, 440.0, out.sample_rate);
            out.set_frames_at_time(current_duration, pcm_frames);
        }
        current_duration += step_duration;
    }
}
