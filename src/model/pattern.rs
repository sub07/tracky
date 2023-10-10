use rust_utils_macro::{EnumIter, EnumValue};

use super::field::{value_object::HexDigit, Field, NoteFieldValue};

macro_rules! declare_field {
    ($($snake_case:ident $pascal_case:ident $size:literal $ty:ty),* $(,)?) => {
        #[derive(Default)]
        pub struct PatternLine {
            $(
                pub $snake_case: Field<$ty>,
            )*
        }

        #[derive(EnumIter, EnumValue)]
        pub enum LineField {
            $(
                #[value(len: usize = $size)]
                $pascal_case,
            )*
        }
    };
}

declare_field! {
    note Note 3 NoteFieldValue,
    velocity Velocity 2 (HexDigit, HexDigit),
    instrument Instrument 2 (HexDigit, HexDigit),
}

impl LineField {
    pub const LINE_LEN: i32 = LineField::line_len() as i32;

    pub const fn line_len() -> usize {
        let mut sum = 0;
        let mut i = 0;
        while i < LineField::size() as i32 {
            sum += LineField::VARIANTS[i as usize].len();
            i += 1;
        }
        sum
    }
}

pub struct Patterns {
    pub lines: Vec<PatternLine>,
    patterns_len: Vec<u32>,
    pub nb_column: u32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    selected_pattern_index: usize,
}

impl Default for Patterns {
    fn default() -> Self {
        let patterns_len = vec![128];
        let nb_column = 20;
        Patterns::new(nb_column, patterns_len)
    }
}

impl Patterns {
    pub fn new(nb_column: u32, patterns_len: Vec<u32>) -> Patterns {
        let initial_capacity = patterns_len.iter().sum::<u32>() * nb_column;
        let initial_capacity = initial_capacity as usize;
        let mut lines = Vec::with_capacity(initial_capacity);
        lines.resize_with(initial_capacity, Default::default);

        Patterns {
            lines,
            patterns_len,
            nb_column,
            cursor_x: 0,
            cursor_y: 0,
            selected_pattern_index: 0,
        }
    }

    fn pattern_range(&self, index: usize) -> anyhow::Result<std::ops::Range<usize>> {
        let start = self.patterns_len[..index].iter().sum::<u32>() as usize;
        let end = start + (self.patterns_len[index] * self.nb_column) as usize;

        anyhow::ensure!(start <= self.lines.len(), "pattern index out of bounds");
        anyhow::ensure!(end <= self.lines.len(), "pattern index out of bounds");

        Ok(start..end)
    }

    fn pattern<'a>(&'a self, index: usize) -> anyhow::Result<PatternView<'a>> {
        Ok(PatternView {
            lines: &self.lines[self.pattern_range(index)?],
            nb_column: self.nb_column,
            len: self.patterns_len[index],
        })
    }

    fn pattern_mut<'a>(&'a mut self, index: usize) -> anyhow::Result<PatternViewMut<'a>> {
        let range = self.pattern_range(index)?;
        Ok(PatternViewMut {
            lines: &mut self.lines[range],
            nb_column: self.nb_column,
            len: self.patterns_len[index],
        })
    }

    pub fn current_pattern<'a>(&'a self) -> PatternView<'a> {
        self.pattern(self.selected_pattern_index).unwrap()
    }

    pub fn current_pattern_mut<'a>(&'a mut self) -> PatternViewMut<'a> {
        self.pattern_mut(self.selected_pattern_index).unwrap()
    }

    pub fn current_pattern_len(&self) -> u32 {
        self.patterns_len[self.selected_pattern_index]
    }

    pub fn current_line_mut(&mut self) -> &mut PatternLine {
        let current_column_index = self.cursor_x / LineField::LINE_LEN;
        let pattern_len = self.current_pattern_len() as usize;
        let cursor_y = self.cursor_y as usize;
        let range = self.pattern_range(self.selected_pattern_index).unwrap();
        let pattern = &mut self.lines[range];
        &mut pattern[current_column_index as usize * pattern_len + cursor_y]
    }
}

#[derive(Clone, Copy)]
pub struct PatternView<'a> {
    lines: &'a [PatternLine],
    pub nb_column: u32,
    pub len: u32,
}

pub struct PatternViewMut<'a> {
    lines: &'a mut [PatternLine],
    pub nb_column: u32,
    pub len: u32,
}

impl<'a> PatternView<'a> {
    pub fn column(&self, index: usize) -> anyhow::Result<ColumnView<'a>> {
        anyhow::ensure!(
            index < self.nb_column as usize,
            "column index out of bounds"
        );

        let start = index * self.len as usize;
        let end = start + self.len as usize;
        let column = &self.lines[start..end];

        Ok(ColumnView { lines: column })
    }

    pub fn columns(&'a self) -> impl Iterator<Item = ColumnView<'a>> {
        (0..self.nb_column as usize).map(move |column_index| self.column(column_index).unwrap())
    }
}

impl<'a> PatternViewMut<'a> {
    pub fn column_mut(&'a mut self, index: usize) -> anyhow::Result<ColumnViewMut<'a>> {
        anyhow::ensure!(
            index < self.nb_column as usize,
            "column index out of bounds"
        );

        let start = index * self.len as usize;
        let end = start + self.len as usize;
        let column = &mut self.lines[start..end];

        Ok(ColumnViewMut { lines: column })
    }
}

#[derive(Clone, Copy)]
pub struct ColumnView<'a> {
    pub lines: &'a [PatternLine],
}

pub struct ColumnViewMut<'a> {
    pub lines: &'a mut [PatternLine],
}
