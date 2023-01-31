use std::slice::Iter;

use crate::model::pattern::column::Column;

pub struct Pattern {
    columns: Vec<Column>,
}

impl Pattern {
    pub fn new(nb_column: i32, column_len: i32) -> Pattern {
        if nb_column == 0 || column_len == 0 { panic!("Invalid column values") }
        if column_len > 999 { panic!("Invalid column len") }
        let mut columns = Vec::new();
        columns.resize_with(nb_column as usize, || Column::new(column_len));

        Pattern {
            columns,
        }
    }

    pub fn column_len(&self) -> i32 {
        self.columns[0].len()
    }

    pub fn nb_columns(&self) -> i32 { self.columns.len() as i32 }

    pub fn iter(&self) -> Iter<Column> {
        self.columns.iter()
    }

    pub fn column(&self, index: i32) -> &Column {
        &self.columns[index as usize]
    }

    pub fn column_mut(&mut self, index: i32) -> &mut Column {
        &mut self.columns[index as usize]
    }
}
