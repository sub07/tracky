use std::collections::HashMap;

use joy_collection_utils::hash_map_of;
use joy_macro::DisplayFromDebug;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::model::{
    pattern::{HexDigit, NoteName, OctaveValue},
    Direction,
};

#[derive(PartialEq, Copy, Clone, Debug, DisplayFromDebug)]
pub enum Action {
    Note {
        note_name: NoteName,
        octave_modifier: i32,
    },
    Hex(HexDigit),
    Octave(OctaveValue),
    ClearField,
    Move(Direction, u32),
    InsertPattern,
    NextPattern,
    PreviousPattern,
    TogglePlay,
    NoteCut,
    ModifyDefaultOctave(i32),
    Exit,
}

impl Action {
    const fn note(n: NoteName, o: i32) -> Action {
        Action::Note {
            note_name: n,
            octave_modifier: o,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum PatternEditInputContext {
    Note,
    Octave,
    Hex,
    Global,
}

#[derive(Debug)]
pub struct KeyBindings {
    context_bindings: HashMap<PatternEditInputContext, HashMap<(KeyModifiers, KeyCode), Action>>,
}

impl KeyBindings {
    pub fn action(
        &self,
        modifiers: KeyModifiers,
        key: KeyCode,
        input_context: PatternEditInputContext,
    ) -> Option<Action> {
        let keyboard_event = (modifiers, key);

        if let Some(key_map) = self.context_bindings.get(&input_context) {
            if let Some(action) = key_map.get(&keyboard_event) {
                return Some(*action);
            } else if let Some(global_keybinds) =
                self.context_bindings.get(&PatternEditInputContext::Global)
            {
                if let Some(action) = global_keybinds.get(&keyboard_event) {
                    return Some(*action);
                }
            }
        }
        None
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let context_bindings = hash_map_of!(
            PatternEditInputContext::Note => hash_map_of!(
                (KeyModifiers::empty(), KeyCode::Char('a')) => Action::note(NoteName::C, 0),
                (KeyModifiers::empty(), KeyCode::Char('é')) => Action::note(NoteName::CSharp, 0),
                (KeyModifiers::empty(), KeyCode::Char('z')) => Action::note(NoteName::D, 0),
                (KeyModifiers::empty(), KeyCode::Char('"')) => Action::note(NoteName::DSharp, 0),
                (KeyModifiers::empty(), KeyCode::Char('e')) => Action::note(NoteName::E, 0),
                (KeyModifiers::empty(), KeyCode::Char('r')) => Action::note(NoteName::F, 0),
                (KeyModifiers::empty(), KeyCode::Char('(')) => Action::note(NoteName::FSharp, 0),
                (KeyModifiers::empty(), KeyCode::Char('t')) => Action::note(NoteName::G, 0),
                (KeyModifiers::empty(), KeyCode::Char('-')) => Action::note(NoteName::GSharp, 0),
                (KeyModifiers::empty(), KeyCode::Char('y')) => Action::note(NoteName::A, 0),
                (KeyModifiers::empty(), KeyCode::Char('è')) => Action::note(NoteName::ASharp, 0),
                (KeyModifiers::empty(), KeyCode::Char('u')) => Action::note(NoteName::B, 0),
                (KeyModifiers::empty(), KeyCode::Char('&')) => Action::NoteCut,
            ),
            PatternEditInputContext::Octave => hash_map_of!(
                (KeyModifiers::empty(), KeyCode::Char('à')) => Action::Octave(OctaveValue::OCTAVE_0),
                (KeyModifiers::empty(), KeyCode::Char('&')) => Action::Octave(OctaveValue::OCTAVE_1),
                (KeyModifiers::empty(), KeyCode::Char('é')) => Action::Octave(OctaveValue::OCTAVE_2),
                (KeyModifiers::empty(), KeyCode::Char('"')) => Action::Octave(OctaveValue::OCTAVE_3),
                (KeyModifiers::empty(), KeyCode::Char('\'')) => Action::Octave(OctaveValue::OCTAVE_4),
                (KeyModifiers::empty(), KeyCode::Char('(')) => Action::Octave(OctaveValue::OCTAVE_5),
                (KeyModifiers::empty(), KeyCode::Char('-')) => Action::Octave(OctaveValue::OCTAVE_6),
                (KeyModifiers::empty(), KeyCode::Char('è')) => Action::Octave(OctaveValue::OCTAVE_7),
                (KeyModifiers::empty(), KeyCode::Char('_')) => Action::Octave(OctaveValue::OCTAVE_8),
                (KeyModifiers::empty(), KeyCode::Char('ç')) => Action::Octave(OctaveValue::OCTAVE_9),
            ),
            PatternEditInputContext::Hex => hash_map_of!(
                (KeyModifiers::empty(), KeyCode::Char('à')) => Action::Hex(HexDigit::HEX_0),
                (KeyModifiers::empty(), KeyCode::Char('&')) => Action::Hex(HexDigit::HEX_1),
                (KeyModifiers::empty(), KeyCode::Char('é')) => Action::Hex(HexDigit::HEX_2),
                (KeyModifiers::empty(), KeyCode::Char('"')) => Action::Hex(HexDigit::HEX_3),
                (KeyModifiers::empty(), KeyCode::Char('\'')) => Action::Hex(HexDigit::HEX_4),
                (KeyModifiers::empty(), KeyCode::Char('(')) => Action::Hex(HexDigit::HEX_5),
                (KeyModifiers::empty(), KeyCode::Char('-')) => Action::Hex(HexDigit::HEX_6),
                (KeyModifiers::empty(), KeyCode::Char('è')) => Action::Hex(HexDigit::HEX_7),
                (KeyModifiers::empty(), KeyCode::Char('_')) => Action::Hex(HexDigit::HEX_8),
                (KeyModifiers::empty(), KeyCode::Char('ç')) => Action::Hex(HexDigit::HEX_9),
                (KeyModifiers::empty(), KeyCode::Char('a')) => Action::Hex(HexDigit::HEX_A),
                (KeyModifiers::empty(), KeyCode::Char('b')) => Action::Hex(HexDigit::HEX_B),
                (KeyModifiers::empty(), KeyCode::Char('c')) => Action::Hex(HexDigit::HEX_C),
                (KeyModifiers::empty(), KeyCode::Char('d')) => Action::Hex(HexDigit::HEX_D),
                (KeyModifiers::empty(), KeyCode::Char('e')) => Action::Hex(HexDigit::HEX_E),
                (KeyModifiers::empty(), KeyCode::Char('f')) => Action::Hex(HexDigit::HEX_F),
            ),
            PatternEditInputContext::Global => hash_map_of!(
                (KeyModifiers::empty(), KeyCode::Down) => Action::Move(Direction::Down, 1),
                (KeyModifiers::empty(), KeyCode::Up) => Action::Move(Direction::Up, 1),
                (KeyModifiers::empty(), KeyCode::Left) => Action::Move(Direction::Left, 1),
                (KeyModifiers::empty(), KeyCode::Right) => Action::Move(Direction::Right, 1),
                (KeyModifiers::empty(), KeyCode::Insert) => Action::InsertPattern,
                (KeyModifiers::empty(), KeyCode::Char('+')) => Action::NextPattern,
                (KeyModifiers::empty(), KeyCode::Char('-')) => Action::PreviousPattern,
                (KeyModifiers::empty(), KeyCode::Delete) => Action::ClearField,
                (KeyModifiers::empty(), KeyCode::Char(' ')) => Action::TogglePlay,
                (KeyModifiers::empty(), KeyCode::Char('*')) => Action::ModifyDefaultOctave(1),
                (KeyModifiers::empty(), KeyCode::Char('/')) => Action::ModifyDefaultOctave(-1),
                (KeyModifiers::empty(), KeyCode::Esc) => Action::Exit,
            ),
        );

        KeyBindings { context_bindings }
    }
}
