use std::slice::Iter;

use crate::model::Direction;
use crate::model::pattern_line::PatternLine;

pub struct Pattern {
    lines: Vec<PatternLine>,
}

impl Pattern {
    pub fn new(length: usize) -> Pattern {
        let mut lines = Vec::new();
        lines.resize_with(length, Default::default);
        Pattern {
            lines,
        }
    }

    pub fn iter(&self) -> Iter<PatternLine> {
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
