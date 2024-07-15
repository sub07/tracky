use joy_value_object::mk_vo;

use super::pattern::{NoteName, OctaveValue};

mk_vo! {
    pub MidiValue: i32,
    default: 69,
    min: 12,
    max: (OctaveValue::MAX_VALUE + 1) * 12 + 12 - 1,
    additional_derive: Hash, Eq
}

pub fn note_to_midi_value(note: NoteName, octave: OctaveValue) -> MidiValue {
    let octave = octave.value();
    let note_index = note.ordinal() as i32;
    (octave * 12 + 12 + note_index).into()
}

impl From<(NoteName, OctaveValue)> for MidiValue {
    fn from((note, octave): (NoteName, OctaveValue)) -> Self {
        note_to_midi_value(note, octave)
    }
}

// f(midi_num) = fixed_note_freq_reference * 2^((midi_value_to_be_converted - fixed_note_midi_value_reference) / 12)
// fixed_note_freq_reference = 440
// fixed_note_midi_value_reference = 69
// midi_value_to_be_converted = midi_value
pub fn midi_to_freq(midi_value: MidiValue) -> f32 {
    let midi_value = midi_value.value() as f32;
    const A4_FREQ: f32 = 440.0;
    const A4_MIDI: f32 = 69.0;

    let b_pow = (midi_value - A4_MIDI) / 12.0;
    let b = 2.0f32.powf(b_pow);

    if true {}

    A4_FREQ * b
}

impl From<MidiValue> for f32 {
    fn from(midi_value: MidiValue) -> Self {
        midi_to_freq(midi_value)
    }
}

#[allow(dead_code)]
pub fn note_to_freq(note: NoteName, octave: OctaveValue) -> f32 {
    midi_to_freq(note_to_midi_value(note, octave))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn a4_should_be_midi_number_69() {
        let note = NoteName::A;
        let octave = OctaveValue::OCTAVE_4;
        assert_eq!(
            MidiValue::new_unchecked(69),
            note_to_midi_value(note, octave),
        );
    }

    #[test]
    fn d6_should_be_midi_number_86() {
        let note = NoteName::D;
        let octave = OctaveValue::OCTAVE_6;
        assert_eq!(
            MidiValue::new_unchecked(86),
            note_to_midi_value(note, octave)
        );
    }

    #[test]
    fn c4_should_be_midi_number_60() {
        let note = NoteName::C;
        let octave = OctaveValue::OCTAVE_4;
        assert_eq!(
            MidiValue::new_unchecked(60),
            note_to_midi_value(note, octave)
        );
    }

    #[test]
    fn test_that_all_note_are_mapped_to_a_midi_value_and_are_all_distinct() {
        let octaves = [
            OctaveValue::OCTAVE_0,
            OctaveValue::OCTAVE_1,
            OctaveValue::OCTAVE_2,
            OctaveValue::OCTAVE_3,
            OctaveValue::OCTAVE_4,
            OctaveValue::OCTAVE_5,
            OctaveValue::OCTAVE_6,
            OctaveValue::OCTAVE_7,
            OctaveValue::OCTAVE_8,
            OctaveValue::OCTAVE_9,
        ];

        let mut midi_values = HashSet::<MidiValue>::new();

        for octave in octaves {
            for note in NoteName::VARIANTS {
                midi_values.insert(note_to_midi_value(note, octave));
            }
        }

        assert_eq!(
            MidiValue::MAX_VALUE - MidiValue::MIN_VALUE + 1, // +1 because it starts at 0
            midi_values.len() as i32,
        );
    }

    #[test]
    fn a4_should_be_freq_440_0() {
        let freq = note_to_freq(NoteName::A, OctaveValue::OCTAVE_4);
        assert_eq!(440.0, freq);
    }

    #[test]
    fn b2_should_be_freq_123_47() {
        let freq = note_to_freq(NoteName::B, OctaveValue::OCTAVE_2);
        approx::assert_relative_eq!(123.47, freq, epsilon = 0.001);
    }
}
