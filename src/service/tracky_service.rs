use iced::{Command, widget::scrollable, Event};

use crate::{Tracky, model::{OctaveValue, NoteValue, Note, pattern::{NoteField, DigitIndex, ColumnLineElement}, value_object::HexDigit}, keybinding};

impl Tracky {
    pub fn get_current_octave_or_default(&self) -> OctaveValue {
        match self.pattern_collection.current_line().note_field.note {
            Some(note) => match note {
                NoteValue::Cut => self.default_octave,
                NoteValue::Note(_, octave) => octave,
            },
            _ => self.default_octave,
        }
    }

    pub fn set_note(&mut self, note: Note) {
        let octave = self.get_current_octave_or_default();
        self.pattern_collection.current_line_mut().note_field =
            NoteField::new(Some(NoteValue::Note(note, octave)));
    }

    pub fn set_velocity(&mut self, hex_digit: HexDigit) {
        let velocity_digit_index = match self.pattern_collection.local_column_index() {
            3 => DigitIndex::First,
            4 => DigitIndex::Second,
            _ => panic!("Should not happen"),
        };

        self.pattern_collection
            .current_line_mut()
            .velocity_field
            .set_digit_hex(velocity_digit_index, hex_digit)
    }

    pub fn set_instrument(&mut self, hex_value: HexDigit) {
        let instr_digit_index = match self.pattern_collection.local_column_index() {
            5 => DigitIndex::First,
            6 => DigitIndex::Second,
            _ => panic!("Should not happen"),
        };

        self.pattern_collection
            .current_line_mut()
            .instrument_field
            .set_digit_hex(instr_digit_index, hex_value)
    }

    pub fn set_hex(&mut self, hex_value: HexDigit) {
        match self.pattern_collection.local_column_index() {
            3 | 4 => self.set_velocity(hex_value),
            5 | 6 => self.set_instrument(hex_value),
            _ => {}
        }
    }

    pub fn set_octave(&mut self, octave: OctaveValue) {
        if let Some(NoteValue::Note(_, field_octave)) =
            &mut self.pattern_collection.current_line_mut().note_field.note
        {
            *field_octave = octave
        }
    }

    pub fn clear(&mut self) {
        match self.pattern_collection.local_column_index() {
            0 | 2 => {
                self.pattern_collection.current_line_mut().note_field = NoteField::default();
                self.pattern_collection
                    .current_line_mut()
                    .velocity_field
                    .clear();
                self.pattern_collection
                    .current_line_mut()
                    .instrument_field
                    .clear();
            }
            3 | 4 => self
                .pattern_collection
                .current_line_mut()
                .velocity_field
                .clear(),
            5 | 6 => self
                .pattern_collection
                .current_line_mut()
                .instrument_field
                .clear(),
            _ => {}
        }
    }

    pub fn move_cursor(
        &mut self,
        x: i32,
        y: i32,
    ) -> Command<<Tracky as iced::Application>::Message> {
        self.pattern_collection.cursor_x += x;
        self.pattern_collection.cursor_y += y;

        if self.pattern_collection.cursor_x % ColumnLineElement::LINE_LEN == 1 {
            self.pattern_collection.cursor_x += x;
        }

        self.pattern_collection.cursor_x = i32::rem_euclid(
            self.pattern_collection.cursor_x,
            ColumnLineElement::LINE_LEN
                * self.pattern_collection.current_pattern().columns.len() as i32,
        );
        self.pattern_collection.cursor_y = i32::rem_euclid(
            self.pattern_collection.cursor_y,
            self.pattern_collection.current_pattern().columns[0]
                .lines
                .len() as i32,
        );

        let cursor_x_column_index = self.pattern_collection.cursor_x / ColumnLineElement::LINE_LEN;

        return scrollable::snap_to(
            self.pattern_scroll_id.clone(),
            scrollable::RelativeOffset {
                x: cursor_x_column_index as f32
                    / (self.pattern_collection.current_pattern().columns.len() - 1) as f32,
                y: self.pattern_collection.cursor_y as f32
                    / (self.pattern_collection.current_pattern().columns[0]
                        .lines
                        .len()
                        - 1) as f32,
            },
        );
    }

    pub fn convert_event_to_action(&self, event: Event) -> Option<keybinding::Action> {
        let input_context = self.pattern_collection.input_type();
        match event {
            Event::Keyboard(kb_event) => match kb_event {
                iced::keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                } => self.keybindings.action(modifiers, key_code, input_context),
                iced::keyboard::Event::KeyReleased {
                    key_code: _,
                    modifiers: _,
                } => None,
                iced::keyboard::Event::CharacterReceived(_) => None,
                iced::keyboard::Event::ModifiersChanged(_) => None,
            },
            Event::Mouse(_) => None,
            Event::Window(_) => None,
            Event::Touch(_) => None,
        }
    }
}
