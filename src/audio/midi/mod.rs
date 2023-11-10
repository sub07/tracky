use crate::model::field::Note;

use self::value_object::MidiNumber;

pub mod value_object {
    use rust_utils::{define_bounded_value_object};

    use crate::model::field::value_object::OctaveValue;

    // From C0 to B(OctaveValue::MAX)
    define_bounded_value_object!(pub MidiNumber, i32, 69, 12, OctaveValue::MAX * 12 + 23);
}

pub trait IntoMidiNumber {
    fn into_midi_note(self) -> value_object::MidiNumber;
}

impl IntoMidiNumber for Note {
    fn into_midi_note(self) -> value_object::MidiNumber {
        let (note, octave) = self;
        let octave = octave.value();
        let note_index = note.ordinal() as i32;
        MidiNumber::new_unchecked(octave * 12 + 12 + note_index) // Note and OctaveValue are bounded accordingly to MidiNumber boundaries so it should not fail if the formula is right
    }
}

#[cfg(test)]
mod tests {

    use crate::model::field::{value_object::OctaveValue, NoteName};

    use super::*;

    #[test]
    fn a4_should_be_midi_number_69() {
        let note = NoteName::A;
        let octave = OctaveValue::OCTAVE_4;
        let midi_number = (note, octave).into_midi_note();
        assert_eq!(69, midi_number.value());
    }

    #[test]
    fn d6_should_be_midi_number_86() {
        let note = NoteName::D;
        let octave = OctaveValue::OCTAVE_6;
        let midi_number = (note, octave).into_midi_note();
        assert_eq!(86, midi_number.value());
    }
}
