use std::time::Duration;

use crate::{
    audio::{
        signal::StereoSignal,
        value_object::{Pan, Volume},
    },
    model::{
        field::{Note, NoteFieldValue},
        pattern::PatternLine,
    },
};

use super::{
    generation::{
        SampleParametersInterpolator, SawWaveDescriptor, SineWaveDescriptor, SquareWaveDescriptor,
    },
    instrument::Instrument,
    IntoFrequency,
};

pub struct AudioChannel {
    buffer: StereoSignal,
    current_volume: Volume,
    current_pan: Pan,
    current_instrument: Option<Instrument>,
    current_note: Option<Note>,
    step_duration: Duration,
}

impl AudioChannel {
    pub fn new(sample_rate: f32, line_per_second: f32) -> AudioChannel {
        let step_duration = AudioChannel::compute_buffer_duration(line_per_second);
        AudioChannel {
            buffer: StereoSignal::new(step_duration, sample_rate),
            current_volume: Volume::FULL,
            current_pan: Pan::CENTER,
            current_instrument: None,
            current_note: None,
            step_duration,
        }
    }

    pub fn compute_buffer_duration(line_per_second: f32) -> Duration {
        Duration::from_secs_f32(1.0 / line_per_second)
    }

    pub fn buffer(&self) -> &StereoSignal {
        &self.buffer
    }

    pub fn process_line(&mut self, line: &PatternLine) {
        if let Some(new_note) = line.note.value() {
            self.current_note = match new_note {
                NoteFieldValue::Note(new_note) => Some(*new_note),
                NoteFieldValue::Cut => None,
            }
        }

        if let Some(new_volume) = line.velocity.get_volume() {
            self.current_volume = new_volume;
        }

        if let Some(new_instrument_index) = line.instrument.get_u8() {
            let need_new_instrument = if let Some(current_instrument) = &mut self.current_instrument
            {
                current_instrument.index != new_instrument_index
            } else {
                true
            };

            if need_new_instrument {
                let new_instrument = match new_instrument_index {
                    0 => Some(Box::new(SampleParametersInterpolator::new(SineWaveDescriptor)) as _),
                    1 => {
                        Some(Box::new(SampleParametersInterpolator::new(SquareWaveDescriptor)) as _)
                    }
                    2 => Some(Box::new(SampleParametersInterpolator::new(SawWaveDescriptor)) as _),
                    3 => Some(Box::new(StereoSignal::from_path("piano.wav").unwrap()) as _),
                    _ => None,
                };

                self.current_instrument = new_instrument.map(|i| Instrument {
                    frame_iter: i,
                    index: new_instrument_index,
                    phase: 0.0,
                });
            }
        }

        if let (Some(instrument), Some(note)) = (&mut self.current_instrument, self.current_note) {
            let line_signal = instrument.frame_iter.collect_for_duration(
                self.step_duration,
                note.into_frequency(),
                self.current_volume,
                self.current_pan,
                &mut instrument.phase,
                self.buffer.sample_rate,
            );

            self.buffer.write_signal(&line_signal).unwrap();
        }
    }
}
