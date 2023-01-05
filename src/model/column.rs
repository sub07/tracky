use std::slice::Iter;

use sdl2::keyboard::Keycode;

use crate::key_bindings::KeyBindings;
use crate::model::column_line::ColumnLine;
use crate::model::Direction;
use crate::model::patterns::PatternsContext;

pub struct Column {
    lines: Vec<ColumnLine>,
}

impl Column {
    pub fn new(length: usize) -> Column {
        let mut lines = Vec::new();
        lines.resize_with(length, Default::default);
        Column {
            lines,
        }
    }

    pub fn iter(&self) -> Iter<ColumnLine> {
        self.lines.iter()
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn move_cursor(&self, local_x_cursor: usize, cursor_y: usize, direction: Direction) -> i32 {
        match direction {
            Direction::Left | Direction::Right => self.lines[cursor_y].move_cursor(local_x_cursor, direction),
            _ => panic!("This function should not be called with this direction")
        }
    }

    pub fn handle_input(&mut self, key: Keycode, key_bindings: &KeyBindings, local_x_cursor: usize, cursor_y: usize, patterns_context: &PatternsContext) {
        self.lines[cursor_y].handle_input(key, key_bindings, local_x_cursor, patterns_context);
    }
}
