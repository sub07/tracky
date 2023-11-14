use crate::{model::field::{
    value_object::{HexDigit, OctaveValue},
    Field, NoteFieldValue, NoteName,
}, audio::value_object::Volume};

impl Field<NoteFieldValue> {
    pub fn set_note_name(&mut self, note: NoteName, default_octave: OctaveValue) {
        match self.value() {
            Some(note_value) => match note_value {
                NoteFieldValue::Note(_) => self.set(NoteFieldValue::Note((note, default_octave))),
                NoteFieldValue::Cut => self.set(NoteFieldValue::Note((note, default_octave))),
            },
            None => self.set(NoteFieldValue::Note((note, default_octave))),
        }
    }

    pub fn set_octave(&mut self, octave: OctaveValue) {
        if let Some(note_value) = self.value() {
            match note_value {
                NoteFieldValue::Note((note, _)) => self.set(NoteFieldValue::Note((*note, octave))),
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

    // Should be elsewhere
    pub fn get_volume(&self) -> Option<Volume> {
        Volume::new(self.get_u8()? as f32 / u8::MAX as f32)
    }

    pub fn set_u8(&mut self, value: u8) {
        let first_digit = HexDigit::new(value / 0x10).unwrap();
        let second_digit = HexDigit::new(value % 0x10).unwrap();
        self.set((first_digit, second_digit));
    }
}
