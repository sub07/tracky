use derive_new::new;
use sdl2::keyboard::Keycode;

use crate::key_bindings::{KeyBindings, PatternValueAction};
use crate::model::field::input_unit::InputUnit;

#[derive(new, Default)]
pub struct NoteField {
    pub note1: InputUnit,
    pub note2: InputUnit,
    pub octave: InputUnit,
}

impl NoteField {
    pub fn handle_input(&mut self, key: Keycode, key_bindings: &KeyBindings, local_x_cursor: usize) {
        match local_x_cursor {
            0 if let Some(action) = key_bindings.note_mapping.get(&key) => {
                let (first_char, second_char) = match action {
                    PatternValueAction::NoteC => ('C', '.'),
                    PatternValueAction::NoteCSharp => ('C', '#'),
                    PatternValueAction::NoteD => ('D', '.'),
                    PatternValueAction::NoteDSharp => ('D', '#'),
                    PatternValueAction::NoteE => ('E', '.'),
                    PatternValueAction::NoteF => ('F', '.'),
                    PatternValueAction::NoteFSharp => ('F', '#'),
                    PatternValueAction::NoteG => ('G', '.'),
                    PatternValueAction::NoteGSharp => ('G', '#'),
                    PatternValueAction::NoteA => ('A', '.'),
                    PatternValueAction::NoteASharp => ('A', '#'),
                    PatternValueAction::NoteB => ('B', '.'),
                    PatternValueAction::ClearUnit => ('.', '.'),
                    _ => (self.note1.value, self.note2.value),
                };
                self.note1.value = first_char;
                self.note2.value = second_char;
            }
            2 if let Some(action) = key_bindings.hex_mapping.get(&key) => {
                self.octave.value = match action {
                    PatternValueAction::Hex0 => '0',
                    PatternValueAction::Hex1 => '1',
                    PatternValueAction::Hex2 => '2',
                    PatternValueAction::Hex3 => '3',
                    PatternValueAction::Hex4 => '4',
                    PatternValueAction::Hex5 => '5',
                    PatternValueAction::Hex6 => '6',
                    PatternValueAction::Hex7 => '7',
                    PatternValueAction::Hex8 => '8',
                    PatternValueAction::Hex9 => '9',
                    PatternValueAction::ClearUnit => '.',
                    _ => self.octave.value,
                };
            }
            1 => panic!("Should not be possible (no cursor on the note alteration)"),
            _ => {},
        }
    }
}
