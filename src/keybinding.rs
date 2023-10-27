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
            } else {
                if let Some(global_keybinds) = self.context_bindings.get(&InputContext::Global) {
                    if let Some(action) = global_keybinds.get(&keyboard_event) {
                        return Some(*action);
                    }
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
                KeyboardEvent::Key(KeyCode::Key0) => Action::Octave(OctaveValue::new(0).unwrap()),
                KeyboardEvent::Key(KeyCode::Key1) => Action::Octave(OctaveValue::new(1).unwrap()),
                KeyboardEvent::Key(KeyCode::Key2) => Action::Octave(OctaveValue::new(2).unwrap()),
                KeyboardEvent::Key(KeyCode::Key3) => Action::Octave(OctaveValue::new(3).unwrap()),
                KeyboardEvent::Key(KeyCode::Key4) => Action::Octave(OctaveValue::new(4).unwrap()),
                KeyboardEvent::Key(KeyCode::Key5) => Action::Octave(OctaveValue::new(5).unwrap()),
                KeyboardEvent::Key(KeyCode::Key6) => Action::Octave(OctaveValue::new(6).unwrap()),
                KeyboardEvent::Key(KeyCode::Key7) => Action::Octave(OctaveValue::new(7).unwrap()),
                KeyboardEvent::Key(KeyCode::Key8) => Action::Octave(OctaveValue::new(8).unwrap()),
                KeyboardEvent::Key(KeyCode::Key9) => Action::Octave(OctaveValue::new(9).unwrap()),
            ),
            InputContext::Hex => hash_map_of!(
                KeyboardEvent::Key(KeyCode::Key0) => Action::Hex(HexDigit::new(0x0).unwrap()),
                KeyboardEvent::Key(KeyCode::Key1) => Action::Hex(HexDigit::new(0x1).unwrap()),
                KeyboardEvent::Key(KeyCode::Key2) => Action::Hex(HexDigit::new(0x2).unwrap()),
                KeyboardEvent::Key(KeyCode::Key3) => Action::Hex(HexDigit::new(0x3).unwrap()),
                KeyboardEvent::Key(KeyCode::Key4) => Action::Hex(HexDigit::new(0x4).unwrap()),
                KeyboardEvent::Key(KeyCode::Key5) => Action::Hex(HexDigit::new(0x5).unwrap()),
                KeyboardEvent::Key(KeyCode::Key6) => Action::Hex(HexDigit::new(0x6).unwrap()),
                KeyboardEvent::Key(KeyCode::Key7) => Action::Hex(HexDigit::new(0x7).unwrap()),
                KeyboardEvent::Key(KeyCode::Key8) => Action::Hex(HexDigit::new(0x8).unwrap()),
                KeyboardEvent::Key(KeyCode::Key9) => Action::Hex(HexDigit::new(0x9).unwrap()),
                KeyboardEvent::Key(KeyCode::A) => Action::Hex(HexDigit::new(0xA).unwrap()),
                KeyboardEvent::Key(KeyCode::B) => Action::Hex(HexDigit::new(0xB).unwrap()),
                KeyboardEvent::Key(KeyCode::C) => Action::Hex(HexDigit::new(0xC).unwrap()),
                KeyboardEvent::Key(KeyCode::D) => Action::Hex(HexDigit::new(0xD).unwrap()),
                KeyboardEvent::Key(KeyCode::E) => Action::Hex(HexDigit::new(0xE).unwrap()),
                KeyboardEvent::Key(KeyCode::F) => Action::Hex(HexDigit::new(0xF).unwrap()),
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
