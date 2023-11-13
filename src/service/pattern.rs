use crate::{
    keybinding::InputContext,
    model::pattern::{PatternLineDescriptor, Patterns},
};

impl Patterns {
    pub fn local_column_index(&self) -> i32 {
        self.cursor_x % PatternLineDescriptor::LINE_LEN
    }

    pub fn input_type(&self) -> InputContext {
        let cursor_x = self.cursor_x % PatternLineDescriptor::LINE_LEN;
        match cursor_x {
            0 => InputContext::Note,
            2 => InputContext::Octave,
            3..=6 => InputContext::Hex,
            _ => panic!("Invalid cursor position: {cursor_x}"),
        }
    }
}
