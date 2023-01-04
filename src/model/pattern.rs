use std::slice::Iter;

use crate::model::{Direction, ColumnLineElement};
use crate::model::column::Column;

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

    pub fn nb_columns(&self) -> usize { self.columns.len() }

    pub fn iter(&self) -> Iter<Column> {
        self.columns.iter()
    }

    pub fn column(&self, index: usize) -> &Column {
        &self.columns[index]
    }
}
