use crate::model::field::Note;

use self::value_object::MidiNumber;

pub mod value_object {
    use rust_utils::define_value_object;

    // From C0 to B8
    define_value_object!(pub MidiNumber, u8, 69, |v| {v >= 12 && v <= (12 * crate::model::field::value_object::MAX_OCTAVE) + 23});
}

pub trait IntoMidiNumber {
    fn into_midi_note(self) -> value_object::MidiNumber;
}

impl IntoMidiNumber for Note {
    fn into_midi_note(self) -> value_object::MidiNumber {
        let (note, octave) = self;
        let octave = octave.value();
        let note_index = note.ordinal() as u8;
        MidiNumber::new(octave * 12 + 12 + note_index).unwrap() // Note and OctaveValue are bounded accordingly to MidiNumber boundaries so it should not fail of the formula is right
    }
}

#[cfg(test)]
mod tests {

    use crate::model::field::{value_object::OctaveValue, NoteName};

    use super::*;

    #[test]
    fn A4_should_be_midi_number_69() {
        let note = NoteName::A;
        let octave = OctaveValue::new(4).unwrap();
        let midi_number = (note, octave).into_midi_note();
        assert_eq!(69, midi_number.value());
    }

    #[test]
    fn D6_should_be_midi_number_86() {
        let note = NoteName::D;
        let octave = OctaveValue::new(6).unwrap();
        let midi_number = (note, octave).into_midi_note();
        assert_eq!(86, midi_number.value());
    }
}
