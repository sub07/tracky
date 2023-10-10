use std::time::Duration;

use crate::audio::{
    generation::{
        SampleParametersInterpolator, SawWaveDescriptor, SineWaveDescriptor, SquareWaveDescriptor,
    },
    model::signal::StereoSignal,
    FrameIterator, IntoFrequency,
};

use super::{
    field::{Note, NoteFieldValue},
    pattern::ColumnView,
};

struct Instrument {
    frame_iter: Box<dyn FrameIterator>,
    index: u8,
    phase: f32,
}

pub struct AudioChannel {
    signal: StereoSignal,
    step_duration: Duration,
    current_instrument: Option<Instrument>,
    current_amp: f32,
    current_note: Option<Note>,
}

impl AudioChannel {
    pub fn new(bps: f32, sample_rate: f32) -> AudioChannel {
        AudioChannel {
            signal: StereoSignal::new(Duration::ZERO, sample_rate),
            step_duration: Duration::from_secs_f32(1.0 / bps),
            current_instrument: None,
            current_amp: 1.0,
            current_note: None,
        }
    }

    pub fn signal(&self) -> &StereoSignal {
        &self.signal
    }

    pub fn handle_column(&mut self, column: ColumnView<'_>) {
        let column_duration = self
            .step_duration
            .checked_mul(column.lines.len() as u32)
            .unwrap();
        if column_duration > self.signal.duration() {
            self.signal.ensure_duration(column_duration);
        }

        let mut current_duration = Duration::ZERO;

        for line in column.lines {
            if let Some(note_value) = line.note.value() {
                match note_value {
                    NoteFieldValue::Note(note) => self.current_note = Some(*note),
                    NoteFieldValue::Cut => self.current_note = None,
                }
            }
            if let Some(new_instrument_index) = line.instrument.get_u8() {
                let instrument = match new_instrument_index {
                    0 => Some(Box::new(SampleParametersInterpolator::new(SineWaveDescriptor)) as _),
                    1 => {
                        Some(Box::new(SampleParametersInterpolator::new(SquareWaveDescriptor)) as _)
                    }
                    2 => Some(Box::new(SampleParametersInterpolator::new(SawWaveDescriptor)) as _),
                    _ => None,
                };
                if let Some(Instrument {
                    frame_iter: _,
                    index,
                    phase,
                }) = &mut self.current_instrument
                {
                    if *index != new_instrument_index {
                        *phase = 0.0;
                    }
                }
                self.current_instrument = instrument.map(|i| Instrument {
                    frame_iter: i,
                    index: new_instrument_index,
                    phase: 0.0,
                });
            }
            if let Some(velocity) = line.velocity.get_u8() {
                self.current_amp = velocity as f32 / u8::MAX as f32;
            }

            match (
                &mut self.current_instrument,
                self.current_note,
                self.current_amp,
            ) {
                (
                    Some(Instrument {
                        frame_iter,
                        index: _,
                        phase,
                    }),
                    Some(note),
                    amp,
                ) => {
                    let signal = frame_iter.collect_for_duration(
                        self.step_duration,
                        note.into_frequency(),
                        amp,
                        phase,
                        self.signal.sample_rate,
                    );

                    self.signal
                        .write_frames_at_duration(current_duration, &signal)
                        .unwrap();
                }
                _ => {}
            }
            current_duration += self.step_duration;
        }
    }
}
