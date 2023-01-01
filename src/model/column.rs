use std::slice::Iter;

use crate::model::Direction;
use crate::model::column_line::ColumnLine;

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
}
