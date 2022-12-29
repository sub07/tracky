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
        if nb_patterns == 0 || pattern_len == 0 { panic!("Invalid patterns value") }
        let mut patterns = Vec::new();
        patterns.resize_with(nb_patterns, || Pattern::new(pattern_len));

        Patterns {
            patterns,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn iter(&self) -> Iter<Pattern> {
        self.patterns.iter()
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.cursor_x == 0 {
                    self.cursor_x = self.patterns.len() * PatternLineElement::LINE_LEN - 1;
                } else {
                    self.cursor_x -= 1;
                }
            }
            Direction::Right => {
                if self.cursor_x == self.patterns.len() * PatternLineElement::LINE_LEN - 1 {
                    self.cursor_x = 0;
                } else {
                    self.cursor_x += 1;
                }
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
