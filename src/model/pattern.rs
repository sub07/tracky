use rust_utils_macro::{EnumIter, EnumValue, New};

use crate::model::{HexValue, NoteField, OctaveValue};

#[derive(Default, Copy, Clone)]
pub struct VelocityField {
    pub value: Option<u8>,
}

impl VelocityField {
    pub fn new(HexValue(digit_1): HexValue, HexValue(digit_2): HexValue) -> VelocityField {
        VelocityField {
            value: Some((digit_1 << 4) | digit_2),
        }
    }
}

#[derive(New, Default, Clone)]
pub struct ColumnLine {
    pub note_field: NoteField,
    pub velocity_field: VelocityField,
}

#[derive(EnumIter, EnumValue)]
pub enum ColumnLineElement {
    #[value(len: usize = 3)]
    Note,
    #[value(len: usize = 2)]
    Velocity,
}

impl ColumnLineElement {
    pub const LINE_LEN: i32 = ColumnLineElement::line_len() as i32;

    pub const fn line_len() -> usize {
        let mut sum = 0;
        let mut i = 0;
        while i < ColumnLineElement::size() as i32 {
            sum += ColumnLineElement::VARIANTS[i as usize].len();
            i += 1;
        }
        sum
    }
}

#[derive(Clone)]
pub struct Column {
    pub lines: Vec<ColumnLine>,
}

const DEFAULT_COLUMN_LEN: usize = 64;

impl Default for Column {
    fn default() -> Self {
        let lines = vec![ColumnLine::default(); DEFAULT_COLUMN_LEN];
        Column { lines }
    }
}

#[derive(Default, Clone)]
pub struct Pattern {
    pub columns: Vec<Column>,
}

pub struct Patterns {
    patterns: Vec<Pattern>,
    pub selected_pattern_index: usize,
    pub column_cursor: i32,
    pub line_cursor: i32,
    pub default_octave: OctaveValue,
}
