use crate::model::field::{
    value_object::{HexDigit, OctaveValue},
    Field, Note, NoteFieldValue, NoteName,
};

impl Field<NoteFieldValue> {
    pub fn set_note_name(&mut self, note: NoteName, default_octave: OctaveValue) {
        match self.value() {
            Some(note_value) => match note_value {
                NoteFieldValue::Note((_, octave)) => {
                    self.set(NoteFieldValue::Note((note, *octave)))
                }
                NoteFieldValue::Cut => self.set(NoteFieldValue::Note((note, default_octave))),
            },
            None => self.set(NoteFieldValue::Note((note, default_octave))),
        }
    }

    pub fn set_octave(&mut self, octave: OctaveValue) {
        match self.value() {
            Some(note_value) => match note_value {
                NoteFieldValue::Note((note, _)) => self.set(NoteFieldValue::Note((*note, octave))),
                NoteFieldValue::Cut => {}
            },
            None => {}
        }
    }
}

impl Field<(HexDigit, HexDigit)> {
    pub fn set_first_digit(&mut self, digit: HexDigit) {
        match self.value() {
            Some((_, second_digit)) => self.set((digit, *second_digit)),
            None => self.set((digit, HexDigit::default())),
        }
    }

    pub fn set_second_digit(&mut self, digit: HexDigit) {
        match self.value() {
            Some((first_digit, _)) => self.set((*first_digit, digit)),
            None => self.set((HexDigit::default(), digit)),
        }
    }

    pub fn get_u8(&self) -> Option<u8> {
        self.value()
            .map(|(first_digit, second_digit)| first_digit.value() * 0x10 + second_digit.value())
    }
}
