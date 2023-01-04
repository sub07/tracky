use derive_new::new;
use sdl2::keyboard::Keycode;

use crate::key_bindings::KeyBindings;
use crate::model::{ColumnLineElement, Direction};
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

    pub fn handle_input(&mut self, key: Keycode, key_bindings: &KeyBindings, local_x_cursor: usize) {
        const MAX_INDEX: usize = ColumnLineElement::LINE_LEN - 1;
        match local_x_cursor {
            0..=2 => self.note.handle_input(key, key_bindings, local_x_cursor),
            3..=MAX_INDEX => self.velocity.handle_input(key, key_bindings, local_x_cursor - ColumnLineElement::Note.len()),
            _ => panic!("Invalid local x cursor : {local_x_cursor}")
        }
    }
}
