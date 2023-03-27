use crate::key_bindings::PatternInputType;
use crate::model::pattern::{ColumnLineElement, Direction};
use crate::model::pattern::column_line::ColumnLine;
use crate::model::pattern::field::note::OctaveValue;
use crate::model::pattern::pattern::Pattern;

pub struct Patterns {
    patterns: Vec<Pattern>,
    pub(crate) selected_pattern_index: usize,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub default_octave: OctaveValue,
}

impl Patterns {
    pub fn new(nb_column: i32, column_len: i32) -> Patterns {
        let initial_pattern = Pattern::new(nb_column, column_len);
        Patterns {
            patterns: vec![initial_pattern],
            selected_pattern_index: 0,
            cursor_x: 0,
            cursor_y: 0,
            default_octave: OctaveValue::new(5),
        }
    }

    pub fn current_pattern(&self) -> &Pattern {
        &self.patterns[self.selected_pattern_index]
    }

    pub fn current_pattern_mut(&mut self) -> &mut Pattern {
        &mut self.patterns[self.selected_pattern_index]
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

                self.cursor_x += remaining_local_x_cursor;
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

    pub fn current_line_mut(&mut self) -> &mut ColumnLine {
        let current_column_index = self.cursor_x / ColumnLineElement::LINE_LEN;
        let cursor_y = self.cursor_y;
        self.current_pattern_mut().column_mut(current_column_index).line_mut(cursor_y)
    }

    pub fn line_local_x_cursor(&self) -> i32 {
        self.cursor_x % ColumnLineElement::LINE_LEN
    }

    pub fn cursor_input_type(&self) -> PatternInputType {
        match self.line_x_cursor() {
            0 => PatternInputType::Note,
            2 => PatternInputType::Octave,
            3 | 4 => PatternInputType::Hex,
            _ => panic!("Should not happen"),
        }
    }

    pub fn line_x_cursor(&self) -> i32 {
        self.cursor_x % ColumnLineElement::LINE_LEN
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
}
