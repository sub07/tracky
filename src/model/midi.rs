use joy_value_object::mk_vo;

use super::pattern::{NoteName, OctaveValue};

mk_vo! {
    pub MidiValue: i32,
    default: 69,
    min: 12,
    max: (OctaveValue::MAX_VALUE + 1) * 12 + 12 - 1,
    additional_derive: Hash, Eq
}

impl From<(NoteName, OctaveValue)> for MidiValue {
    fn from(value: (NoteName, OctaveValue)) -> Self {
        let (note, octave) = value;
        let octave = octave.value();
        let note_index = note.ordinal() as i32;
        (octave * 12 + 12 + note_index).into()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn a4_should_be_midi_number_69() {
        let note = NoteName::A;
        let octave = OctaveValue::OCTAVE_4;
        assert_eq!(MidiValue::new_unchecked(69), (note, octave).into());
    }

    #[test]
    fn d6_should_be_midi_number_86() {
        let note = NoteName::D;
        let octave = OctaveValue::OCTAVE_6;
        assert_eq!(MidiValue::new_unchecked(86), (note, octave).into());
    }

    #[test]
    fn c4_should_be_midi_number_60() {
        let note = NoteName::C;
        let octave = OctaveValue::OCTAVE_4;
        assert_eq!(MidiValue::new_unchecked(60), (note, octave).into());
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
                midi_values.insert((note, octave).into());
            }
        }

        assert_eq!(
            MidiValue::MAX_VALUE - MidiValue::MIN_VALUE + 1, // +1 because it starts at 0
            midi_values.len() as i32,
        );
    }
}
