use std::time::Duration;

use crate::audio::{
    generation::{
        SampleParametersInterpolator, SawWaveDescriptor, SineWaveDescriptor, SquareWaveDescriptor,
    },
    signal::StereoSignal,
    value_object::{Pan, Volume},
    FrameIterator, IntoFrequency,
};

use super::{
    field::{Note, NoteFieldValue},
    pattern::PatternLine,
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
    current_amp: Volume,
    current_note: Option<Note>,
    current_duration: Duration,
}

impl AudioChannel {
    pub fn new(bps: f32, sample_rate: f32, pattern_len: u32) -> AudioChannel {
        let (step_duration, signal_duration) = AudioChannel::compute_duration(bps, pattern_len);
        AudioChannel {
            signal: StereoSignal::new(signal_duration, sample_rate),
            step_duration,
            current_instrument: None,
            current_amp: Volume::default(),
            current_note: None,
            current_duration: Duration::ZERO,
        }
    }

    pub fn compute_duration(bps: f32, pattern_len: u32) -> (Duration, Duration) {
        let step_duration = Duration::from_secs_f32(1.0 / bps);
        (
            step_duration,
            step_duration.checked_mul(pattern_len).unwrap(),
        )
    }

    pub fn signal(&self) -> &StereoSignal {
        &self.signal
    }

    pub fn handle_lines(&mut self, lines: &[PatternLine]) {
        for line in lines {
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
                self.current_amp = Volume::new(velocity as f32 / u8::MAX as f32).unwrap();
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
                        Pan::default(),
                        phase,
                        self.signal.sample_rate,
                    );

                    self.signal
                        .write_frames_at_duration(self.current_duration, &signal)
                        .unwrap();
                }
                _ => {}
            }
            self.current_duration += self.step_duration;
        }
    }
}
