use std::time::Duration;

use crate::model::pattern::Column;

use super::{
    generation::{SawWaveDescriptor, SineWaveDescriptor, SquareWaveDescriptor},
    pcm_sample::PcmStereoSample,
    Samples,
};

pub struct AudioChannelBuilder {
    data: PcmStereoSample,
    current_instrument: Option<Box<dyn Samples>>,
}

impl AudioChannelBuilder {
    pub fn handle_column(&mut self, column: &Column) {
        let bps = 6.0;
        let step_duration = Duration::from_secs_f64(1.0 / bps);
        let mut current_duration = Duration::ZERO;
        let mut out_audio = self.data.frames.iter_mut();

        for line in &column.lines {
            if let Some(instrument_index) = line.instrument_field.value {
                let instrument = match instrument_index {
                    0 => Some(Box::new(SineWaveDescriptor::new(1.0)) as Box<dyn Samples>),
                    1 => Some(Box::new(SquareWaveDescriptor::new(1.0)) as Box<dyn Samples>),
                    2 => Some(Box::new(SawWaveDescriptor::new(1.0)) as Box<dyn Samples>),
                    _ => None,
                };
                self.current_instrument = instrument;
            }
            if let Some(instrument) = self.current_instrument.as_mut() {
                let pcm_frames = instrument.collect_for_duration(step_duration, 440.0, self.data.sample_rate);
                out_audio
            }
            current_duration += step_duration;
        }
    }
}
