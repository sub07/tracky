use crate::model::{
    pattern::{
        Field, HexDigit, NoteFieldValue, NoteName, OctaveValue, PatternLineDescriptor, Patterns,
    },
    Direction,
};

impl Field<NoteFieldValue> {
    pub fn set_note_name(&mut self, note: NoteName, octave: OctaveValue) {
        match self.value() {
            Some(note_value) => match note_value {
                NoteFieldValue::Note(_) => self.set(NoteFieldValue::Note((note, octave))),
                NoteFieldValue::Cut => self.set(NoteFieldValue::Note((note, octave))),
            },
            None => self.set(NoteFieldValue::Note((note, octave))),
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

    pub fn set_u8(&mut self, value: u8) {
        let first_digit = HexDigit::new(value / 0x10).unwrap();
        let second_digit = HexDigit::new(value % 0x10).unwrap();
        self.set((first_digit, second_digit));
    }
}

impl Patterns {
    pub fn move_cursor(&mut self, direction: Direction, step: u32) {
        let step = step as i32;
        let (x, y) = direction.vector();
        let direction_vector = (x * step, y * step);
        match direction_vector {
            // Vertical
            (0, d) => {
                self.current_row += d;
                self.current_row = self.current_row.rem_euclid(self.channel_len);
            }
            // Horizontal
            (d, 0) => {
                self.current_field += d;

                self.current_channel += self
                    .current_field
                    .div_euclid(PatternLineDescriptor::LINE_LEN);

                self.current_field = self
                    .current_field
                    .rem_euclid(PatternLineDescriptor::LINE_LEN);
                self.current_channel = self.current_channel.rem_euclid(self.channel_count);
            }
            _ => unreachable!(),
        }
    }

    pub fn modify_default_octave(&mut self, modifier: i32) {
        self.default_octave = self.default_octave + modifier;
    }

    pub fn set_note(&mut self, note: NoteName, octave_modifier: i32) {
        let default_octave = self.default_octave + octave_modifier;
        self.current_line_mut()
            .note
            .set_note_name(note, default_octave);
    }

    pub fn set_octave(&mut self, octave: OctaveValue) {
        self.current_line_mut().note.set_octave(octave);
    }

    pub fn set_hex(&mut self, digit: HexDigit) {
        match PatternLineDescriptor::field_by_cursor(self.current_field) {
            PatternLineDescriptor::Velocity => set_double_hex_field(
                self.current_field,
                &mut self.current_line_mut().velocity,
                digit,
            ),
            PatternLineDescriptor::Instrument => set_double_hex_field(
                self.current_field,
                &mut self.current_line_mut().instrument,
                digit,
            ),
            _ => unreachable!(),
        }
    }

    pub fn clear(&mut self) {
        match PatternLineDescriptor::field_by_cursor(self.current_field) {
            PatternLineDescriptor::Note => {
                self.current_line_mut().note.clear();
                self.current_line_mut().velocity.clear();
                self.current_line_mut().instrument.clear();
            }
            PatternLineDescriptor::Velocity => self.current_line_mut().velocity.clear(),
            PatternLineDescriptor::Instrument => self.current_line_mut().instrument.clear(),
        }
    }

    pub fn set_note_cut(&mut self) {
        self.current_line_mut().note.set(NoteFieldValue::Cut);
    }
}

fn set_double_hex_field(
    field_cursor: i32,
    field: &mut Field<(HexDigit, HexDigit)>,
    digit: HexDigit,
) {
    match PatternLineDescriptor::local_field_cursor(field_cursor) {
        0 => field.set_first_digit(digit),
        1 => field.set_second_digit(digit),
        _ => unreachable!(),
    }
}
