use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::key_bindings::{KeyBindings, PatternInputUnitAction};
use crate::model::{Direction, PatternInputType};
use crate::model::field::note::OctaveValue;
use crate::model::field::velocity::HexValue;
use crate::model::patterns::Patterns;

#[derive(Default)]
pub struct PatternsController {
    key_bindings: KeyBindings,
}

impl PatternsController {
    pub fn handle_event(&self, model: &mut Patterns, event: Event) {
        match event {
            Event::KeyDown { keycode, .. } if let Some(keycode) = keycode => {
                match keycode {
                    Keycode::Down => model.move_cursor(Direction::Down),
                    Keycode::Up => model.move_cursor(Direction::Up),
                    Keycode::Left => model.move_cursor(Direction::Left),
                    Keycode::Right => model.move_cursor(Direction::Right),
                    Keycode::Insert => model.insert_pattern(),
                    Keycode::KpMinus => model.navigate_to_previous_pattern(),
                    Keycode::KpPlus => model.navigate_to_next_pattern(),
                    keycode => self.handle_other_keycode(model, keycode),
                }
            }
            _ => {}
        }
    }

    fn handle_other_keycode(&self, model: &mut Patterns, keycode: Keycode) {
        let input_type = model.cursor_input_type();
        let action = self.key_bindings.context_bindings
            .get(&input_type)
            .expect("All enum field should be in map")
            .get(&keycode)
            .copied();
        if let Some(action) = action {
            match input_type {
                PatternInputType::Note => Self::handle_note_input(model, action),
                PatternInputType::Octave => Self::handle_octave_input(model, action),
                PatternInputType::Hex => Self::handle_hex_input(model, action),
            }
        }
    }

    fn handle_hex_input(model: &mut Patterns, action: PatternInputUnitAction) {
        let hex = match action {
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
            PatternInputUnitAction::HexA => Some(0xA),
            PatternInputUnitAction::HexB => Some(0xB),
            PatternInputUnitAction::HexC => Some(0xC),
            PatternInputUnitAction::HexD => Some(0xD),
            PatternInputUnitAction::HexE => Some(0xE),
            PatternInputUnitAction::HexF => Some(0xF),
            PatternInputUnitAction::ClearUnit => None,
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

    fn handle_octave_input(model: &mut Patterns, action: PatternInputUnitAction) {
        let octave = match action {
            PatternInputUnitAction::Octave0 => Some(0u8),
            PatternInputUnitAction::Octave1 => Some(1),
            PatternInputUnitAction::Octave2 => Some(2),
            PatternInputUnitAction::Octave3 => Some(3),
            PatternInputUnitAction::Octave4 => Some(4),
            PatternInputUnitAction::Octave5 => Some(5),
            PatternInputUnitAction::Octave6 => Some(6),
            PatternInputUnitAction::Octave7 => Some(7),
            PatternInputUnitAction::Octave8 => Some(8),
            PatternInputUnitAction::Octave9 => Some(9),
            PatternInputUnitAction::ClearUnit => None,
            _ => panic!("Should not happen"),
        };

        if let Some(octave) = octave {
            let octave = OctaveValue::new(octave);
            model.current_line_mut().note.set_octave(octave);
        } else {
            model.current_line_mut().note.clear();
        }
    }

    fn handle_note_input(model: &mut Patterns, action: PatternInputUnitAction) {
        match action {
            PatternInputUnitAction::ClearUnit => model.current_line_mut().note.clear(),
            action if let Ok(note) = action.try_into() => {
                let default_octave = model.default_octave;
                model.current_line_mut().note.set_note(note, default_octave);
            }
            _ => panic!("Should not happen")
        }
    }
}