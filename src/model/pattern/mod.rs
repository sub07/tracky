use rust_utils_macro::{EnumIter, EnumValue};

use crate::key_bindings::Action;

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

impl Direction {
    pub fn from_action(action: &Action) -> Direction {
        match action {
            Action::MoveRight => Direction::Right,
            Action::MoveLeft => Direction::Left,
            Action::MoveUp => Direction::Up,
            Action::MoveDown => Direction::Down,
            _ => panic!("Should not attempt to convert action {action:?} to direction"),
        }
    }
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
