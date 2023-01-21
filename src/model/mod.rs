use rust_utils_macro::{EnumIter, EnumValue};

pub mod column_line;
pub mod column;
pub mod pattern;
pub mod field;
pub mod patterns;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(EnumIter, EnumValue)]
pub enum ColumnLineElement {
    #[value(len: usize = 3)]
    Note,
    #[value(len: usize = 2)]
    Velocity,
}

impl ColumnLineElement {
    pub const LINE_LEN: i32 = ColumnLineElement::line_len();

    const fn line_len() -> i32 {
        let mut sum = 0;
        let mut i = 0;
        while i < ColumnLineElement::size() as i32 {
            sum += ColumnLineElement::VARIANTS[i as usize].len() as i32;
            i += 1;
        }
        sum
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum PatternInputType {
    Note,
    Octave,
    Hex,
}
