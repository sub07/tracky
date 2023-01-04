use derive_new::new;
use sdl2::keyboard::Keycode;

use crate::key_bindings::{KeyBindings, PatternValueAction};
use crate::model::field::input_unit::InputUnit;

#[derive(new, Default)]
pub struct VelocityField {
    pub digit1: InputUnit,
    pub digit2: InputUnit,
}

impl VelocityField {
    pub fn handle_input(&mut self, key: Keycode, key_bindings: &KeyBindings, local_x_cursor: usize) {
        if let Some(action) = key_bindings.hex_mapping.get(&key) {
            let new_char = match action {
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
                PatternValueAction::HexA => 'A',
                PatternValueAction::HexB => 'B',
                PatternValueAction::HexC => 'C',
                PatternValueAction::HexD => 'D',
                PatternValueAction::HexE => 'E',
                PatternValueAction::HexF => 'F',
                PatternValueAction::ClearUnit => '.',
                _ if local_x_cursor == 0 => self.digit1.value,
                _ if local_x_cursor == 1 => self.digit2.value,
                _ => panic!("Invalid local x cursor : {local_x_cursor}"),
            };

            if local_x_cursor == 0 {
                self.digit1.value = new_char;
            } else if local_x_cursor == 1 {
                self.digit2.value = new_char;
            } else {
                panic!("Invalid local x cursor : {local_x_cursor}")
            }
        }
    }
}


