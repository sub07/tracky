use std::slice::Iter;

use crate::model::pattern::column_line::ColumnLine;
use crate::model::pattern::Direction;

pub struct Column {
    lines: Vec<ColumnLine>,
}

impl Column {
    pub fn new(length: i32) -> Column {
        let mut lines = Vec::new();
        lines.resize_with(length as usize, Default::default);
        Column {
            lines,
        }
    }

    pub fn iter(&self) -> Iter<ColumnLine> {
        self.lines.iter()
    }

    pub fn len(&self) -> i32 {
        self.lines.len() as i32
    }

    pub fn move_cursor(&self, local_x_cursor: i32, cursor_y: i32, direction: Direction) -> i32 {
        match direction {
            Direction::Left | Direction::Right => self.lines[cursor_y as usize].move_cursor(local_x_cursor, direction),
            _ => panic!("This function should not be called with this direction")
        }
    }

    pub fn line_mut(&mut self, index: i32) -> &mut ColumnLine {
        &mut self.lines[index as usize]
    }
}
