use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::key_bindings::{Action, KeyBindings, PatternInputType};
use crate::model::pattern::Direction;
use crate::model::pattern::field::note::OctaveValue;
use crate::model::pattern::field::velocity::HexValue;
use crate::model::pattern::patterns::Patterns;

#[derive(Default)]
pub struct PatternsController {
    key_bindings: KeyBindings,
}

impl PatternsController {
    pub fn handle_event(&self, model: &mut Patterns, event: Event) {
        match event {
            Event::KeyDown { keycode, .. } if let Some(keycode) = keycode => {
                let action = self.key_bindings.context_bindings
                    .get(&PatternInputType::Global).expect("Missing global from keybindings context")
                    .get(&keycode).copied();
                if let Some(action) = action {
                    match action {
                        Action::MoveDown | Action::MoveUp => {}
                        Action::MoveLeft | Action::MoveRight => {
                            let direction = Direction::from_action(&action);

                            model.move_cursor(direction);
                        }
                        Action::InsertPattern => model.insert_pattern(),
                        Action::NextPattern => model.navigate_to_next_pattern(),
                        Action::PreviousPattern => model.navigate_to_previous_pattern(),
                        _ => todo!()
                    }
                } else {
                    self.handle_other_keycode(model, keycode)
                }
            }
            _ => {}
        }
    }

    fn handle_other_keycode(&self, model: &mut Patterns, keycode: Keycode) {
        let input_type = model.cursor_input_type();
        let action = self.key_bindings.context_bindings
            .get(&input_type).expect("Wrong key binding context")
            .get(&keycode).copied();
        if let Some(action) = action {
            match input_type {
                PatternInputType::Note => Self::handle_note_input(model, action),
                PatternInputType::Octave => Self::handle_octave_input(model, action),
                PatternInputType::Hex => Self::handle_hex_input(model, action),
                _ => panic!("Should not happen")
            }
        }
    }

    fn handle_hex_input(model: &mut Patterns, action: Action) {
        let hex = match action {
            Action::Hex0 => Some(0u8),
            Action::Hex1 => Some(1),
            Action::Hex2 => Some(2),
            Action::Hex3 => Some(3),
            Action::Hex4 => Some(4),
            Action::Hex5 => Some(5),
            Action::Hex6 => Some(6),
            Action::Hex7 => Some(7),
            Action::Hex8 => Some(8),
            Action::Hex9 => Some(9),
            Action::HexA => Some(0xA),
            Action::HexB => Some(0xB),
            Action::HexC => Some(0xC),
            Action::HexD => Some(0xD),
            Action::HexE => Some(0xE),
            Action::HexF => Some(0xF),
            Action::ClearUnit => None,
            _ => panic!("Should not happen"),
        };

        let line_local_x_cursor = model.line_local_x_cursor();

        match line_local_x_cursor {
            3 | 4 => {
                Self::handle_velocity_input(model, hex, line_local_x_cursor);
            }
            _ => panic!("Should not happen"),
        }
    }

    fn handle_velocity_input(model: &mut Patterns, hex: Option<u8>, line_local_x_cursor: i32) {
        if let Some(hex) = hex {
            let hex = HexValue::new(hex);
            if line_local_x_cursor == 3 {
                model.current_line_mut().velocity.set_first_digit_hex(hex)
            } else {
                model.current_line_mut().velocity.set_second_digit_hex(hex)
            }
        } else {
            model.current_line_mut().velocity.clear();
        }
    }

    fn handle_octave_input(model: &mut Patterns, action: Action) {
        let octave = match action {
            Action::Octave0 => Some(0u8),
            Action::Octave1 => Some(1),
            Action::Octave2 => Some(2),
            Action::Octave3 => Some(3),
            Action::Octave4 => Some(4),
            Action::Octave5 => Some(5),
            Action::Octave6 => Some(6),
            Action::Octave7 => Some(7),
            Action::Octave8 => Some(8),
            Action::Octave9 => Some(9),
            Action::ClearUnit => None,
            _ => panic!("Should not happen"),
        };

        if let Some(octave) = octave {
            let octave = OctaveValue::new(octave);
            model.current_line_mut().note.set_octave(octave);
        } else {
            model.current_line_mut().note.clear();
        }
    }

    fn handle_note_input(model: &mut Patterns, action: Action) {
        match action {
            Action::ClearUnit => model.current_line_mut().note.clear(),
            action if let Ok(note) = action.try_into() => {
                let default_octave = model.default_octave;
                model.current_line_mut().note.set_note(note, default_octave);
            }
            _ => panic!("Should not happen")
        }
    }
}
