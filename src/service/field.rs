use crate::model::pattern::{
    Field, HexDigit, NoteFieldValue, NoteName, OctaveValue, PatternLineDescriptor,
};

impl Field<NoteFieldValue> {
    pub fn set_note_name(&mut self, note: NoteName, octave: OctaveValue) {
        match self.value() {
            Some(note_value) => match note_value {
                NoteFieldValue::Note(_, _) => self.set(NoteFieldValue::Note(note, octave)),
                NoteFieldValue::Cut => self.set(NoteFieldValue::Note(note, octave)),
            },
            None => self.set(NoteFieldValue::Note(note, octave)),
        }
    }

    pub fn set_octave(&mut self, octave: OctaveValue) {
        if let Some(note_value) = self.value() {
            match note_value {
                NoteFieldValue::Note(note, _) => self.set(NoteFieldValue::Note(*note, octave)),
                NoteFieldValue::Cut => {}
            }
        }
    }
}

impl Field<(HexDigit, HexDigit)> {
    pub fn set_first_digit(&mut self, digit: HexDigit) {
        match self.value() {
            Some((_, second_digit)) => self.set((digit, *second_digit)),
            None => self.set((digit, HexDigit::DEFAULT)),
        }
    }

    pub fn set_second_digit(&mut self, digit: HexDigit) {
        match self.value() {
            Some((first_digit, _)) => self.set((*first_digit, digit)),
            None => self.set((HexDigit::DEFAULT, digit)),
        }
    }

    pub fn get_u8(&self) -> Option<u8> {
        self.value()
            .map(|(first_digit, second_digit)| first_digit.value() * 0x10 + second_digit.value())
    }

    pub fn get_percentage(&self) -> Option<f32> {
        self.get_u8().map(|hex| hex as f32 / u8::MAX as f32)
    }

    pub fn set_by_index(&mut self, field_index: i32, value: HexDigit) {
        match PatternLineDescriptor::local_field_cursor(field_index) {
            0 => self.set_first_digit(value),
            1 => self.set_second_digit(value),
            _ => unreachable!(),
        }
    }
}
