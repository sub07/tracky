use rust_utils_macro::{EnumIter, EnumValue, New};

use crate::{
    keybinding::PatternInputType,
    model::{HexValue, NoteField, OctaveValue},
};

#[derive(Default, Copy, Clone)]
pub struct VelocityField {
    pub value: Option<u8>,
}

pub enum HexDigit {
    First,
    Second,
}

impl VelocityField {
    pub fn new(HexValue(digit_1): HexValue, HexValue(digit_2): HexValue) -> VelocityField {
        VelocityField {
            value: Some((digit_1 << 4) | digit_2),
        }
    }

    pub fn set_digit_hex(&mut self, digit_index: HexDigit, HexValue(digit): HexValue) {
        let (mask, value) = match digit_index {
            HexDigit::First => (0x0F, digit << 4),
            HexDigit::Second => (0xF0, digit),
        };

        let mut current_value = self.value.unwrap_or(0);
        current_value &= mask;
        current_value |= value;

        self.value = Some(current_value);
    }

    pub fn set_first_digit_hex(&mut self, value: HexValue) {
        self.set_digit_hex(HexDigit::First, value);
    }

    pub fn set_second_digit_hex(&mut self, value: HexValue) {
        self.set_digit_hex(HexDigit::Second, value);
    }

    pub fn clear(&mut self) {
        self.value = None;
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

impl Column {
    pub fn line_mut(&mut self, index: i32) -> &mut ColumnLine {
        &mut self.lines[index as usize]
    }

    pub fn line(&self, index: i32) -> &ColumnLine {
        &self.lines[index as usize]
    }
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

impl Pattern {
    pub fn column_mut(&mut self, index: i32) -> &mut Column {
        &mut self.columns[index as usize]
    }

    pub fn column(&self, index: i32) -> &Column {
        &self.columns[index as usize]
    }
}

pub struct PatternCollection {
    patterns: Vec<Pattern>,
    pub selected_pattern_index: usize,
    pub column_cursor: i32,
    pub line_cursor: i32,
    pub default_octave: OctaveValue,
}

impl PatternCollection {
    pub fn input_type(&self) -> PatternInputType {
        let cursor_x = self.column_cursor % ColumnLineElement::LINE_LEN;
        match cursor_x {
            0 => PatternInputType::Note,
            2 => PatternInputType::Octave,
            3 | 4 => PatternInputType::Hex,
            _ => panic!("Invalid cursor position: {cursor_x}"),
        }
    }

    pub fn current_pattern_mut(&mut self) -> &mut Pattern {
        &mut self.patterns[self.selected_pattern_index]
    }

    pub fn current_pattern(&self) -> &Pattern {
        &self.patterns[self.selected_pattern_index]
    }

    pub fn current_line_mut(&mut self) -> &mut ColumnLine {
        let current_column_index = self.column_cursor / ColumnLineElement::LINE_LEN;
        let cursor_y = self.line_cursor;
        self.current_pattern_mut()
            .column_mut(current_column_index)
            .line_mut(cursor_y)
    }

    pub fn current_line(&self) -> &ColumnLine {
        let current_column_index = self.column_cursor / ColumnLineElement::LINE_LEN;
        let cursor_y = self.line_cursor;
        self.current_pattern()
            .column(current_column_index)
            .line(cursor_y)
    }

    pub fn local_column_index(&self) -> i32 {
        self.column_cursor % ColumnLineElement::LINE_LEN
    }
}

impl Default for PatternCollection {
    fn default() -> Self {
        let pattern = Pattern {
            columns: vec![Column::default(); 15],
        };
        Self {
            patterns: vec![pattern],
            selected_pattern_index: Default::default(),
            column_cursor: Default::default(),
            line_cursor: Default::default(),
            default_octave: Default::default(),
        }
    }
}
