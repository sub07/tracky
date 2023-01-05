use std::slice::Iter;

use sdl2::keyboard::Keycode;

use crate::key_bindings::KeyBindings;
use crate::model::column::Column;
use crate::model::ColumnLineElement;
use crate::model::patterns::PatternsContext;

pub struct Pattern {
    columns: Vec<Column>,
}

impl Pattern {
    pub fn new(nb_column: usize, column_len: usize) -> Pattern {
        if nb_column == 0 || column_len == 0 { panic!("Invalid column values") }
        if column_len > 999 { panic!("Invalid column len") }
        let mut columns = Vec::new();
        columns.resize_with(nb_column, || Column::new(column_len));

        Pattern {
            columns,
        }
    }

    pub fn column_len(&self) -> usize {
        self.columns[0].len()
    }

    pub fn handle_input(&mut self, key: Keycode, key_bindings: &KeyBindings, cursor_x: usize, cursor_y: usize, patterns_context: &PatternsContext) {
        let current_column_index = cursor_x / ColumnLineElement::LINE_LEN;
        let local_x_cursor = cursor_x % ColumnLineElement::LINE_LEN;
        self.columns[current_column_index].handle_input(key, key_bindings, local_x_cursor, cursor_y, patterns_context);
    }

    pub fn nb_columns(&self) -> usize { self.columns.len() }

    pub fn iter(&self) -> Iter<Column> {
        self.columns.iter()
    }

    pub fn column(&self, index: usize) -> &Column {
        &self.columns[index]
    }
}
