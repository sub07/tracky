use derive_new::new;
use sdl2::keyboard::Keycode;

use crate::key_bindings::{KeyBindings, PatternInputUnitAction};
use crate::model::patterns::PatternsContext;

#[derive(new, Default)]
pub struct VelocityField {
    pub value: Option<u8>,
}

impl VelocityField {
    pub fn handle_input(&mut self, key: Keycode, key_bindings: &KeyBindings, local_x_cursor: usize, _: &PatternsContext) {
        match key_bindings.hex_mapping.get(&key) {
            Some(action) => {
                let value = match action {
                    PatternInputUnitAction::Hex0 => Some(0u8),
                    PatternInputUnitAction::Hex1 => Some(1),
                    PatternInputUnitAction::Hex2 => Some(2),
                    PatternInputUnitAction::Hex3 => Some(3),
                    PatternInputUnitAction::Hex4 => Some(4),
                    PatternInputUnitAction::Hex5 => Some(5),
                    PatternInputUnitAction::Hex6 => Some(6),
                    PatternInputUnitAction::Hex7 => Some(7),
                    PatternInputUnitAction::Hex8 => Some(8),
                    PatternInputUnitAction::Hex9 => Some(9),
                    PatternInputUnitAction::HexA => Some(10),
                    PatternInputUnitAction::HexB => Some(11),
                    PatternInputUnitAction::HexC => Some(12),
                    PatternInputUnitAction::HexD => Some(13),
                    PatternInputUnitAction::HexE => Some(14),
                    PatternInputUnitAction::HexF => Some(15),
                    PatternInputUnitAction::ClearUnit => None,
                    _ => return,
                };

                match value {
                    None => {
                        self.value = None;
                    }
                    Some(mut value) => {
                        let mut current = match self.value {
                            Some(value) => {
                                value
                            }
                            None => {
                                0
                            }
                        };

                        let mask = match local_x_cursor {
                            0 => {
                                value <<= 4;
                                0x0F
                            }
                            1 => 0xF0,
                            _ => panic!("Invalid cursor position: {local_x_cursor}"),
                        };

                        current &= mask;
                        current |= value;

                        self.value = Some(current);
                    }
                }
            }
            None => {
                println!("Invalid key({key}) binding for this cursor position({local_x_cursor}) for velocity");
            }
        }
    }
}


