use derive_new::new;

use crate::model::{Direction, ColumnLineElement};
use crate::model::field::note::NoteField;
use crate::model::field::velocity::VelocityField;

#[derive(new, Default)]
pub struct ColumnLine {
    pub note: NoteField,
    pub velocity: VelocityField,
}

impl ColumnLine {
    pub fn move_cursor(&self, local_x_cursor: usize, direction: Direction) -> i32 {
        match direction {
            Direction::Left => {
                -match local_x_cursor {
                    0 => 1,
                    1 => panic!("Should not be possible (no cursor on the note alteration)"),
                    2 => 2,
                    3..=ColumnLineElement::LINE_LEN => 1,
                    _ => panic!("Not in pattern range"),
                }
            }
            Direction::Right => {
                match local_x_cursor {
                    0 => 2,
                    1 => panic!("Should not be possible (no cursor on the note alteration)"),
                    2..=ColumnLineElement::LINE_LEN => 1,
                    _ => panic!("Not in pattern range"),
                }
            }
            _ => panic!("This function should not be called with this direction")
        }
    }
}
