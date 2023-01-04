use sdl2::keyboard::Keycode;

use crate::key_bindings::KeyBindings;
use crate::model::{ColumnLineElement, Direction};
use crate::model::pattern::Pattern;

pub struct Patterns {
    patterns: Vec<Pattern>,
    pub(crate) selected_pattern_index: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl Patterns {
    pub fn new(nb_column: usize, column_len: usize) -> Patterns {
        let initial_pattern = Pattern::new(nb_column, column_len);
        Patterns {
            patterns: vec![initial_pattern],
            selected_pattern_index: 0,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn current_pattern(&self) -> &Pattern {
        &self.patterns[self.selected_pattern_index]
    }

    pub fn nb_patterns(&self) -> usize { self.patterns.len() }

    pub fn move_cursor(&mut self, direction: Direction) {
        let pattern = self.current_pattern();
        match direction {
            Direction::Left | Direction::Right => {
                let current_column_index = self.cursor_x / ColumnLineElement::LINE_LEN;
                let local_x_cursor = self.cursor_x % ColumnLineElement::LINE_LEN;
                let cursor_y = self.cursor_y;
                let new_local_x_cursor = pattern.column(current_column_index).move_cursor(local_x_cursor, cursor_y, direction);

                let remaining_local_x_cursor = if self.cursor_x == pattern.nb_columns() * ColumnLineElement::LINE_LEN - 1 && new_local_x_cursor > 0 {
                    self.cursor_x = 0;
                    new_local_x_cursor - 1
                } else if self.cursor_x == 0 && new_local_x_cursor < 0 {
                    self.cursor_x = pattern.nb_columns() * ColumnLineElement::LINE_LEN - 1;
                    new_local_x_cursor + 1
                } else { new_local_x_cursor };

                self.cursor_x = (self.cursor_x as i32 + remaining_local_x_cursor) as usize;
            }
            Direction::Up => {
                if self.cursor_y == 0 {
                    self.cursor_y = pattern.column_len() - 1;
                } else {
                    self.cursor_y -= 1;
                }
            }
            Direction::Down => {
                if self.cursor_y == pattern.column_len() - 1 {
                    self.cursor_y = 0;
                } else {
                    self.cursor_y += 1;
                }
            }
        }
    }

    pub fn insert_pattern(&mut self) {
        let nb_column = self.current_pattern().nb_columns();
        let column_len = self.current_pattern().column_len();
        let new_pattern = Pattern::new(nb_column, column_len);

        self.patterns.insert(self.selected_pattern_index + 1, new_pattern);
        self.selected_pattern_index += 1;
    }

    pub fn navigate_to_next_pattern(&mut self) {
        self.selected_pattern_index = if self.selected_pattern_index == self.nb_patterns() - 1 {
            0
        } else {
            self.selected_pattern_index + 1
        }
    }

    pub fn navigate_to_previous_pattern(&mut self) {
        self.selected_pattern_index = if self.selected_pattern_index == 0 {
            self.nb_patterns() - 1
        } else {
            self.selected_pattern_index - 1
        }
    }

    pub fn handle_input(&mut self, key: Keycode, key_bindings: &KeyBindings) {
        self.patterns[self.selected_pattern_index].handle_input(key, key_bindings, self.cursor_x, self.cursor_y);
    }
}