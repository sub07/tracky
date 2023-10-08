use iced::{widget::scrollable, Command, Event};

use crate::{
    keybinding,
    model::{
        field::{
            value_object::{HexDigit, OctaveValue},
            NoteName,
        },
        pattern::LineField,
    },
    Tracky,
};

impl Tracky {
    pub fn set_note_name(&mut self, note: NoteName) {
        self.patterns
            .current_line_mut()
            .note
            .set_note_name(note, self.default_octave)
    }

    pub fn set_velocity(&mut self, digit: HexDigit) {
        match self.patterns.local_column_index() {
            3 => self
                .patterns
                .current_line_mut()
                .velocity
                .set_first_digit(digit),
            4 => self
                .patterns
                .current_line_mut()
                .velocity
                .set_second_digit(digit),
            _ => panic!("Should not happen"),
        };
    }

    pub fn set_instrument(&mut self, digit: HexDigit) {
        match self.patterns.local_column_index() {
            5 => self
                .patterns
                .current_line_mut()
                .instrument
                .set_first_digit(digit),
            6 => self
                .patterns
                .current_line_mut()
                .instrument
                .set_second_digit(digit),
            _ => panic!("Should not happen"),
        };
    }

    pub fn set_hex(&mut self, hex_value: HexDigit) {
        match self.patterns.local_column_index() {
            3 | 4 => self.set_velocity(hex_value),
            5 | 6 => self.set_instrument(hex_value),
            _ => {}
        }
    }

    pub fn set_octave(&mut self, octave: OctaveValue) {
        self.patterns.current_line_mut().note.set_octave(octave);
    }

    pub fn clear(&mut self) {
        match self.patterns.local_column_index() {
            0 | 2 => {
                self.patterns.current_line_mut().note.clear();
                self.patterns.current_line_mut().velocity.clear();
                self.patterns.current_line_mut().instrument.clear();
            }
            3 | 4 => self.patterns.current_line_mut().velocity.clear(),
            5 | 6 => self.patterns.current_line_mut().instrument.clear(),
            _ => {}
        }
    }

    pub fn move_cursor(
        &mut self,
        x: i32,
        y: i32,
    ) -> Command<<Tracky as iced::Application>::Message> {
        self.patterns.cursor_x += x;
        self.patterns.cursor_y += y;

        let (pattern_len, nb_column) = {
            let current_pattern = self.patterns.current_pattern();
            (current_pattern.len, current_pattern.nb_column)
        };

        if self.patterns.cursor_x % LineField::LINE_LEN == 1 {
            self.patterns.cursor_x += x;
        }

        self.patterns.cursor_x = i32::rem_euclid(
            self.patterns.cursor_x,
            LineField::LINE_LEN * nb_column as i32,
        );
        self.patterns.cursor_y = i32::rem_euclid(self.patterns.cursor_y, pattern_len as i32);

        let cursor_x_column_index = self.patterns.cursor_x / LineField::LINE_LEN;

        return scrollable::snap_to(
            self.pattern_scroll_id.clone(),
            scrollable::RelativeOffset {
                x: cursor_x_column_index as f32 / (nb_column - 1) as f32,
                y: self.patterns.cursor_y as f32 / (pattern_len - 1) as f32,
            },
        );
    }

    pub fn convert_event_to_action(&self, event: Event) -> Option<keybinding::Action> {
        let input_context = self.patterns.input_type();
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
