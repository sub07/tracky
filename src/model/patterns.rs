use std::slice::Iter;

use crate::model::{Direction, PATTERN_LINE_LEN};
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
        let cursor_x = self.cursor_x;
        let cursor_y = self.cursor_y;
        let current_pattern_index = cursor_x / PATTERN_LINE_LEN;
        // Refactor to skip an input unit for the note sharp for ex, delegate skip down to note field maybe (or closest)
        // Maybe set note len to 2 for PATTERN_LINE_LEN, if so, check for the guards constants in the draw impl
        if self.patterns[current_pattern_index].move_overflows(cursor_x % PATTERN_LINE_LEN, cursor_y, &direction) {
            match direction {
                Direction::Left => {
                    if current_pattern_index == 0 {
                        self.cursor_x = (self.patterns.len() * PATTERN_LINE_LEN) - 1;
                    }
                    self.cursor_x -= 1;
                }
                Direction::Right => {
                    if current_pattern_index == self.patterns.len() - 1 {
                        self.cursor_x = 0;
                    }
                    self.cursor_x += 1;
                }
                Direction::Up => {
                    self.cursor_y = self.patterns[0].len();
                }
                Direction::Down => {
                    self.cursor_y = 0;
                }
            };
        } else {
            let (x, y) = direction.vec();
            self.cursor_x = (self.cursor_x as i32 + x) as usize;
            self.cursor_y += (self.cursor_y as i32 + y) as usize;
        }
    }
}
