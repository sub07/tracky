use std::collections::HashMap;

use iced::keyboard::{KeyCode, Modifiers};
use rust_utils::hash_map_of;

use crate::model::{
    field::{
        value_object::{HexDigit, OctaveValue},
        NoteName,
    },
    Direction,
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Action {
    Note(NoteName),
    Hex(HexDigit),
    Octave(OctaveValue),
    ClearUnit,
    Move(Direction),
    InsertPattern,
    NextPattern,
    PreviousPattern,
    TogglePlay,
    SetNoteCut,
    ModifyDefaultOctave(i32),
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum InputContext {
    Note,
    Octave,
    Hex,
    Global,
}

pub struct KeyBindings {
    context_bindings: HashMap<InputContext, HashMap<KeyboardEvent, Action>>,
}

#[derive(Hash, PartialEq, Eq)]
enum KeyboardEvent {
    Key(KeyCode),
    ModifierKey(Modifiers, KeyCode),
}

impl KeyBindings {
    pub fn action(
        &self,
        modifiers: Modifiers,
        keycode: KeyCode,
        input_context: InputContext,
    ) -> Option<Action> {
        let keyboard_event = if modifiers.is_empty() {
            KeyboardEvent::Key(keycode)
        } else {
            KeyboardEvent::ModifierKey(modifiers, keycode)
        };

        if let Some(key_map) = self.context_bindings.get(&input_context) {
            if let Some(action) = key_map.get(&keyboard_event) {
                return Some(*action);
            } else if let Some(global_keybinds) = self.context_bindings.get(&InputContext::Global) {
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
            InputContext::Note => hash_map_of!(
                KeyboardEvent::Key(KeyCode::A) => Action::Note(NoteName::C),
                KeyboardEvent::Key(KeyCode::Key2) => Action::Note(NoteName::CSharp),
                KeyboardEvent::Key(KeyCode::Z) => Action::Note(NoteName::D),
                KeyboardEvent::Key(KeyCode::Key3) => Action::Note(NoteName::DSharp),
                KeyboardEvent::Key(KeyCode::E) => Action::Note(NoteName::E),
                KeyboardEvent::Key(KeyCode::R) => Action::Note(NoteName::F),
                KeyboardEvent::Key(KeyCode::Key5) => Action::Note(NoteName::FSharp),
                KeyboardEvent::Key(KeyCode::T) => Action::Note(NoteName::G),
                KeyboardEvent::Key(KeyCode::Key6) => Action::Note(NoteName::GSharp),
                KeyboardEvent::Key(KeyCode::Y) => Action::Note(NoteName::A),
                KeyboardEvent::Key(KeyCode::Key7) => Action::Note(NoteName::ASharp),
                KeyboardEvent::Key(KeyCode::U) => Action::Note(NoteName::B),
                KeyboardEvent::Key(KeyCode::Key1) => Action::SetNoteCut,
            ),
            InputContext::Octave => hash_map_of!(
                KeyboardEvent::Key(KeyCode::Key0) => Action::Octave(OctaveValue::OCTAVE_0),
                KeyboardEvent::Key(KeyCode::Key1) => Action::Octave(OctaveValue::OCTAVE_1),
                KeyboardEvent::Key(KeyCode::Key2) => Action::Octave(OctaveValue::OCTAVE_2),
                KeyboardEvent::Key(KeyCode::Key3) => Action::Octave(OctaveValue::OCTAVE_3),
                KeyboardEvent::Key(KeyCode::Key4) => Action::Octave(OctaveValue::OCTAVE_4),
                KeyboardEvent::Key(KeyCode::Key5) => Action::Octave(OctaveValue::OCTAVE_5),
                KeyboardEvent::Key(KeyCode::Key6) => Action::Octave(OctaveValue::OCTAVE_6),
                KeyboardEvent::Key(KeyCode::Key7) => Action::Octave(OctaveValue::OCTAVE_7),
                KeyboardEvent::Key(KeyCode::Key8) => Action::Octave(OctaveValue::OCTAVE_8),
                KeyboardEvent::Key(KeyCode::Key9) => Action::Octave(OctaveValue::OCTAVE_9),
            ),
            InputContext::Hex => hash_map_of!(
                KeyboardEvent::Key(KeyCode::Key0) => Action::Hex(HexDigit::HEX_0),
                KeyboardEvent::Key(KeyCode::Key1) => Action::Hex(HexDigit::HEX_1),
                KeyboardEvent::Key(KeyCode::Key2) => Action::Hex(HexDigit::HEX_2),
                KeyboardEvent::Key(KeyCode::Key3) => Action::Hex(HexDigit::HEX_3),
                KeyboardEvent::Key(KeyCode::Key4) => Action::Hex(HexDigit::HEX_4),
                KeyboardEvent::Key(KeyCode::Key5) => Action::Hex(HexDigit::HEX_5),
                KeyboardEvent::Key(KeyCode::Key6) => Action::Hex(HexDigit::HEX_6),
                KeyboardEvent::Key(KeyCode::Key7) => Action::Hex(HexDigit::HEX_7),
                KeyboardEvent::Key(KeyCode::Key8) => Action::Hex(HexDigit::HEX_8),
                KeyboardEvent::Key(KeyCode::Key9) => Action::Hex(HexDigit::HEX_9),
                KeyboardEvent::Key(KeyCode::A) => Action::Hex(HexDigit::HEX_A),
                KeyboardEvent::Key(KeyCode::B) => Action::Hex(HexDigit::HEX_B),
                KeyboardEvent::Key(KeyCode::C) => Action::Hex(HexDigit::HEX_C),
                KeyboardEvent::Key(KeyCode::D) => Action::Hex(HexDigit::HEX_D),
                KeyboardEvent::Key(KeyCode::E) => Action::Hex(HexDigit::HEX_E),
                KeyboardEvent::Key(KeyCode::F) => Action::Hex(HexDigit::HEX_F),
            ),
            InputContext::Global => hash_map_of!(
                KeyboardEvent::Key(KeyCode::Down) => Action::Move(Direction::Down),
                KeyboardEvent::Key(KeyCode::Up) => Action::Move(Direction::Up),
                KeyboardEvent::Key(KeyCode::Left) => Action::Move(Direction::Left),
                KeyboardEvent::Key(KeyCode::Right) => Action::Move(Direction::Right),
                KeyboardEvent::Key(KeyCode::Insert) => Action::InsertPattern,
                KeyboardEvent::Key(KeyCode::Plus) => Action::NextPattern,
                KeyboardEvent::Key(KeyCode::Minus) => Action::PreviousPattern,
                KeyboardEvent::Key(KeyCode::Delete) => Action::ClearUnit,
                KeyboardEvent::Key(KeyCode::Space) => Action::TogglePlay,
                KeyboardEvent::Key(KeyCode::NumpadMultiply) => Action::ModifyDefaultOctave(1),
                KeyboardEvent::Key(KeyCode::NumpadDivide) => Action::ModifyDefaultOctave(-1),
            ),
        );

        // KeyCode::Asterisk
        // KeyCode::

        KeyBindings { context_bindings }
    }
}
