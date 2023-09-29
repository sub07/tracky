use std::time::Duration;

use crate::model::{pattern::Column, value_object::OctaveValue, Note};

use super::{
    generation::{
        SampleParametersInterpolator, SawWaveDescriptor, SineWaveDescriptor, SquareWaveDescriptor,
    },
    signal::StereoSignal,
    FrameIterator, IntoFrequency,
};

pub fn note_to_multiplier(semitone: i32) -> f64 {
    const NOTE_MUL: f64 = 1.0594630943593;
    NOTE_MUL.powi(semitone)
}

pub fn handle_column(bps: f64, out: &mut StereoSignal, column: &Column) {
    let mut current_instrument: Option<(Box<dyn FrameIterator>, u8)> = None;
    let step_duration = Duration::from_secs_f64(1.0 / bps);
    let mut current_duration = Duration::ZERO;
    let mut phase = 0.0;
    let mut current_amp = 1.0;
    let mut current_note: Option<(Note, OctaveValue)> = None;

    for line in &column.lines {
        if let Some(note_value) = line.note_field.note {
            match note_value {
                crate::model::NoteValue::Note(note, octave) => current_note = Some((note, octave)),
                crate::model::NoteValue::Cut => current_note = None,
            }
        }
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
