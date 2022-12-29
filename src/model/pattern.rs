use std::slice::Iter;

use crate::model::{Direction, PatternLineElement};
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

    pub(super) fn move_overflows(&mut self, pattern_local_x: usize, cursor_y: usize, direction: &Direction) -> bool {
        match direction {
            Direction::Left => pattern_local_x == 0,
            Direction::Right => pattern_local_x == PatternLineElement::LINE_LEN - 1,
            Direction::Up => cursor_y == 0,
            Direction::Down => cursor_y == self.lines.len() - 1,
        }
    }
}
