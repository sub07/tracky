use std::slice::Iter;

use crate::model::{Direction, PatternLineElement};
use crate::model::pattern::Pattern;

pub struct Patterns {
    patterns: Vec<Pattern>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl Patterns {
    pub fn new(nb_patterns: usize, pattern_len: usize) -> Patterns {
        if nb_patterns == 0 || pattern_len == 0 { panic!("Invalid patterns lengths") }
        if pattern_len > 999 { panic!("Invalid pattern len") }
        let mut patterns = Vec::new();
        patterns.resize_with(nb_patterns, || Pattern::new(pattern_len));

        Patterns {
            patterns,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn pattern_len(&self) -> usize {
        self.patterns[0].len()
    }

    pub fn iter(&self) -> Iter<Pattern> {
        self.patterns.iter()
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left | Direction::Right => {
                let current_pattern_index = self.cursor_x / PatternLineElement::LINE_LEN;
                let local_x_cursor = self.cursor_x % PatternLineElement::LINE_LEN;
                let cursor_y = self.cursor_y;
                let new_local_x_cursor = self.patterns[current_pattern_index].move_cursor(local_x_cursor, cursor_y, direction);

                let remaining_local_x_cursor = if self.cursor_x == self.patterns.len() * PatternLineElement::LINE_LEN - 1 && new_local_x_cursor > 0 {
                    self.cursor_x = 0;
                    new_local_x_cursor - 1
                } else if self.cursor_x == 0 && new_local_x_cursor < 0 {
                    self.cursor_x = self.patterns.len() * PatternLineElement::LINE_LEN - 1;
                    new_local_x_cursor + 1
                } else { new_local_x_cursor };

                self.cursor_x = (self.cursor_x as i32 + remaining_local_x_cursor) as usize;
            }
            Direction::Up => {
                if self.cursor_y == 0 {
                    self.cursor_y = self.patterns[0].len() - 1;
                } else {
                    self.cursor_y -= 1;
                }
            }
            Direction::Down => {
                if self.cursor_y == self.patterns[0].len() - 1 {
                    self.cursor_y = 0;
                } else {
                    self.cursor_y += 1;
                }
            }
        }
    }
}
