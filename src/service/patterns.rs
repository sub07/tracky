use crate::{
    keybinding::InputContext,
    model::pattern::{LineField, PatternLine, Patterns},
};

impl Patterns {
    pub fn current_line(&mut self) -> &PatternLine {
        let current_column_index = self.cursor_x / LineField::LINE_LEN;
        &self
            .current_pattern()
            .column(current_column_index as usize)
            .unwrap()
            .lines[self.cursor_y as usize]
    }

    pub fn local_column_index(&self) -> i32 {
        self.cursor_x % LineField::LINE_LEN
    }

    pub fn input_type(&self) -> InputContext {
        let cursor_x = self.cursor_x % LineField::LINE_LEN;
        match cursor_x {
            0 => InputContext::Note,
            2 => InputContext::Octave,
            3 | 4 | 5 | 6 => InputContext::Hex,
            _ => panic!("Invalid cursor position: {cursor_x}"),
        }
    }
}
