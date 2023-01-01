use std::slice::Iter;

use crate::model::{Direction, ColumnLineElement};
use crate::model::column::Column;

pub struct Pattern {
    columns: Vec<Column>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl Pattern {
    pub fn new(nb_column: usize, column_len: usize) -> Pattern {
        if nb_column == 0 || column_len == 0 { panic!("Invalid column values") }
        if column_len > 999 { panic!("Invalid column len") }
        let mut columns = Vec::new();
        columns.resize_with(nb_column, || Column::new(column_len));

        Pattern {
            columns,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn pattern_len(&self) -> usize {
        self.columns[0].len()
    }

    pub fn iter(&self) -> Iter<Column> {
        self.columns.iter()
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left | Direction::Right => {
                let current_pattern_index = self.cursor_x / ColumnLineElement::LINE_LEN;
                let local_x_cursor = self.cursor_x % ColumnLineElement::LINE_LEN;
                let cursor_y = self.cursor_y;
                let new_local_x_cursor = self.columns[current_pattern_index].move_cursor(local_x_cursor, cursor_y, direction);

                let remaining_local_x_cursor = if self.cursor_x == self.columns.len() * ColumnLineElement::LINE_LEN - 1 && new_local_x_cursor > 0 {
                    self.cursor_x = 0;
                    new_local_x_cursor - 1
                } else if self.cursor_x == 0 && new_local_x_cursor < 0 {
                    self.cursor_x = self.columns.len() * ColumnLineElement::LINE_LEN - 1;
                    new_local_x_cursor + 1
                } else { new_local_x_cursor };

                self.cursor_x = (self.cursor_x as i32 + remaining_local_x_cursor) as usize;
            }
            Direction::Up => {
                if self.cursor_y == 0 {
                    self.cursor_y = self.columns[0].len() - 1;
                } else {
                    self.cursor_y -= 1;
                }
            }
            Direction::Down => {
                if self.cursor_y == self.columns[0].len() - 1 {
                    self.cursor_y = 0;
                } else {
                    self.cursor_y += 1;
                }
            }
        }
    }
}
