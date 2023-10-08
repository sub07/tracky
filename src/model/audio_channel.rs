use std::time::Duration;

use crate::{audio::{model::signal::StereoSignal, FrameIterator, generation::{SampleParametersInterpolator, SineWaveDescriptor, SquareWaveDescriptor, SawWaveDescriptor}, IntoFrequency}};

use super::{pattern::ColumnView, field::{NoteFieldValue, Note}};

// struct Instrument {
//     frame_iter: Box<dyn FrameIterator>,
//     index: u8,
//     phase: f32,
// }

// pub struct AudioChannel {
//     pub data: StereoSignal,
//     step_duration: Duration,
//     current_step: Duration,
//     current_instrument: Option<Instrument>,
//     current_amp: f32,
//     current_note: Option<(Note, OctaveValue)>,
// }

// impl AudioChannel {
//     pub fn new(bps: f32) -> AudioChannel {
//         AudioChannel { data: StereoSignal::, step_duration: (), current_step: (), current_instrument: (), current_amp: (), current_note: () }
//     }
// }

pub fn handle_column(bps: f64, out: &mut StereoSignal, column: ColumnView<'_>) {
    let mut current_instrument: Option<(Box<dyn FrameIterator>, u8)> = None;
    let step_duration = Duration::from_secs_f64(1.0 / bps);
    let mut current_duration = Duration::ZERO;
    let mut phase = 0.0;
    let mut current_amp = 1.0;
    let mut current_note: Option<Note> = None;

    for line in column.lines {
        if let Some(note_value) = line.note.value() {
            match note_value {
                NoteFieldValue::Note(note) => current_note = Some(*note),
                NoteFieldValue::Cut => current_note = None,
            }
        }
        if let Some(new_instrument_index) = line.instrument.get_u8() {
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
        if let Some(velocity) = line.velocity.get_u8() {
            current_amp = velocity as f32 / u8::MAX as f32;
        }

        match (&mut current_instrument, current_note, current_amp) {
            (Some((instrument, _)), Some(note), amp) => {
                let signal = instrument.collect_for_duration(
                    step_duration,
                    note.into_frequency(),
                    amp,
                    &mut phase,
                    out.sample_rate,
                );

                out.write_frames_at_duration(current_duration, &signal)
                    .unwrap();
            }
            _ => {}
        }
        current_duration += step_duration;
    }
}
