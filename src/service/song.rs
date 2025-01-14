use joy_vector::Vector;

use crate::model::{
    pattern::{Field, HexDigit, NoteFieldValue, NoteName, OctaveValue, PatternLineDescriptor},
    song::{self},
    Direction,
};

impl song::State {
    pub fn handle_event(&mut self, event: song::Event) {
        match event {
            song::Event::MutateGlobalOctave { increment } => self.mutate_global_octave(increment),
            song::Event::SetNoteField {
                note,
                octave_modifier,
            } => self.set_note_field(note, octave_modifier),
            song::Event::MoveCursor(direction) => self.move_cursor(direction),
            song::Event::SetNoteFieldToCut => self.set_note_field_to_cut(),
            song::Event::ClearField => self.clear_field(),
            song::Event::SetOctaveField(octave) => self.set_octave_field(octave),
            song::Event::SetHexField(digit) => self.set_hex_field(digit),
            song::Event::NewPattern => todo!(),
            song::Event::NextPattern => todo!(),
            song::Event::PreviousPattern => todo!(),
        }
    }

    fn mutate_global_octave(&mut self, increment: i32) {
        self.global_octave = self.global_octave + increment;
    }

    fn set_note_field(&mut self, note: NoteName, octave_modifier: i32) {
        let line = self.patterns.current_line_mut();
        line.note
            .set_note_name(note, self.global_octave + octave_modifier);
        if line.instrument.value().is_none() {
            line.instrument.set((HexDigit::HEX_0, HexDigit::HEX_0));
        }
    }

    fn move_cursor(&mut self, direction: Direction) {
        match direction.vector() {
            // Vertical
            Vector([0, d]) => {
                self.patterns.current_row += d;
                self.patterns.current_row = self
                    .patterns
                    .current_row
                    .rem_euclid(self.patterns.channel_len);
            }
            // Horizontal
            Vector([d, 0]) => {
                self.patterns.current_field += d;

                self.patterns.current_channel += self
                    .patterns
                    .current_field
                    .div_euclid(PatternLineDescriptor::LINE_LEN);

                self.patterns.current_field = self
                    .patterns
                    .current_field
                    .rem_euclid(PatternLineDescriptor::LINE_LEN);
                self.patterns.current_channel = self
                    .patterns
                    .current_channel
                    .rem_euclid(self.patterns.channel_count);
            }
            _ => unreachable!(),
        }
    }

    fn set_note_field_to_cut(&mut self) {
        self.patterns
            .current_line_mut()
            .note
            .set(NoteFieldValue::Cut);
    }

    fn clear_field(&mut self) {
        let field_cursor = self.patterns.current_field;
        let line = self.patterns.current_line_mut();
        match PatternLineDescriptor::field_by_cursor(field_cursor) {
            PatternLineDescriptor::Note => {
                line.note.clear();
                line.velocity.clear();
                line.instrument.clear();
            }
            PatternLineDescriptor::Velocity => line.velocity.clear(),
            PatternLineDescriptor::Instrument => line.instrument.clear(),
        }
    }

    fn set_octave_field(&mut self, octave: OctaveValue) {
        self.patterns.current_line_mut().note.set_octave(octave);
    }

    pub fn set_hex_field(&mut self, digit: HexDigit) {
        match PatternLineDescriptor::field_by_cursor(self.patterns.current_field) {
            PatternLineDescriptor::Velocity => set_double_hex_field(
                self.patterns.current_field,
                &mut self.patterns.current_line_mut().velocity,
                digit,
            ),
            PatternLineDescriptor::Instrument => set_double_hex_field(
                self.patterns.current_field,
                &mut self.patterns.current_line_mut().instrument,
                digit,
            ),
            _ => unreachable!(),
        }
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
