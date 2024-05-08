use std::collections::HashMap;

use iced::keyboard::{key, Key, Modifiers};
use rust_utils::hash_map_of;
use smol_str::SmolStr;

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
    Key(Key),
    ModifierKey(Modifiers, Key),
}

impl KeyBindings {
    pub fn action(
        &self,
        modifiers: Modifiers,
        key: Key,
        input_context: InputContext,
    ) -> Option<Action> {
        let keyboard_event = if modifiers.is_empty() {
            KeyboardEvent::Key(key)
        } else {
            KeyboardEvent::ModifierKey(modifiers, key)
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

const fn char_key(s: &str) -> Key {
    Key::Character(SmolStr::new_inline(s))
}

macro_rules! gen_char_key {
    ($($name:ident $val:literal),+ $(,)?) => {
        $(
            const $name: Key = char_key($val);
        )+
    };
}

gen_char_key! {
    KEY_A "a", KEY_B "b", KEY_C "c", KEY_D "d", KEY_E "e", KEY_F "f", KEY_G "g", KEY_H "h", KEY_I "i", KEY_J "j", KEY_K "k", KEY_L "l", KEY_M "m", KEY_N "n", KEY_O "o", KEY_P "p", KEY_Q "q", KEY_R "r", KEY_S "s", KEY_T "t", KEY_U "u", KEY_V "v", KEY_W "w", KEY_X "x", KEY_Y "y", KEY_Z "z", KEY_0 "0", KEY_1 "1", KEY_2 "2", KEY_3 "3", KEY_4 "4", KEY_5 "5", KEY_6 "6", KEY_7 "7", KEY_8 "8", KEY_9 "9", KEY_PLUS "+", KEY_MULTIPLY "*", KEY_DIVIDE "/", KEY_MINUS "-",
}

impl Default for KeyBindings {
    fn default() -> Self {
        let context_bindings = hash_map_of!(
            InputContext::Note => hash_map_of!(
                KeyboardEvent::Key(KEY_A) => Action::Note(NoteName::C),
                KeyboardEvent::Key(KEY_2) => Action::Note(NoteName::CSharp),
                KeyboardEvent::Key(KEY_Z) => Action::Note(NoteName::D),
                KeyboardEvent::Key(KEY_3) => Action::Note(NoteName::DSharp),
                KeyboardEvent::Key(KEY_E) => Action::Note(NoteName::E),
                KeyboardEvent::Key(KEY_R) => Action::Note(NoteName::F),
                KeyboardEvent::Key(KEY_5) => Action::Note(NoteName::FSharp),
                KeyboardEvent::Key(KEY_T) => Action::Note(NoteName::G),
                KeyboardEvent::Key(KEY_6) => Action::Note(NoteName::GSharp),
                KeyboardEvent::Key(KEY_Y) => Action::Note(NoteName::A),
                KeyboardEvent::Key(KEY_7) => Action::Note(NoteName::ASharp),
                KeyboardEvent::Key(KEY_U) => Action::Note(NoteName::B),
                KeyboardEvent::Key(KEY_1) => Action::SetNoteCut,
            ),
            InputContext::Octave => hash_map_of!(
                KeyboardEvent::Key(KEY_0) => Action::Octave(OctaveValue::OCTAVE_0),
                KeyboardEvent::Key(KEY_1) => Action::Octave(OctaveValue::OCTAVE_1),
                KeyboardEvent::Key(KEY_2) => Action::Octave(OctaveValue::OCTAVE_2),
                KeyboardEvent::Key(KEY_3) => Action::Octave(OctaveValue::OCTAVE_3),
                KeyboardEvent::Key(KEY_4) => Action::Octave(OctaveValue::OCTAVE_4),
                KeyboardEvent::Key(KEY_5) => Action::Octave(OctaveValue::OCTAVE_5),
                KeyboardEvent::Key(KEY_6) => Action::Octave(OctaveValue::OCTAVE_6),
                KeyboardEvent::Key(KEY_7) => Action::Octave(OctaveValue::OCTAVE_7),
                KeyboardEvent::Key(KEY_8) => Action::Octave(OctaveValue::OCTAVE_8),
                KeyboardEvent::Key(KEY_9) => Action::Octave(OctaveValue::OCTAVE_9),
            ),
            InputContext::Hex => hash_map_of!(
                KeyboardEvent::Key(KEY_0) => Action::Hex(HexDigit::HEX_0),
                KeyboardEvent::Key(KEY_1) => Action::Hex(HexDigit::HEX_1),
                KeyboardEvent::Key(KEY_2) => Action::Hex(HexDigit::HEX_2),
                KeyboardEvent::Key(KEY_3) => Action::Hex(HexDigit::HEX_3),
                KeyboardEvent::Key(KEY_4) => Action::Hex(HexDigit::HEX_4),
                KeyboardEvent::Key(KEY_5) => Action::Hex(HexDigit::HEX_5),
                KeyboardEvent::Key(KEY_6) => Action::Hex(HexDigit::HEX_6),
                KeyboardEvent::Key(KEY_7) => Action::Hex(HexDigit::HEX_7),
                KeyboardEvent::Key(KEY_8) => Action::Hex(HexDigit::HEX_8),
                KeyboardEvent::Key(KEY_9) => Action::Hex(HexDigit::HEX_9),
                KeyboardEvent::Key(KEY_A) => Action::Hex(HexDigit::HEX_A),
                KeyboardEvent::Key(KEY_B) => Action::Hex(HexDigit::HEX_B),
                KeyboardEvent::Key(KEY_C) => Action::Hex(HexDigit::HEX_C),
                KeyboardEvent::Key(KEY_D) => Action::Hex(HexDigit::HEX_D),
                KeyboardEvent::Key(KEY_E) => Action::Hex(HexDigit::HEX_E),
                KeyboardEvent::Key(KEY_F) => Action::Hex(HexDigit::HEX_F),
            ),
            InputContext::Global => hash_map_of!(
                KeyboardEvent::Key(Key::Named(key::Named::ArrowDown)) => Action::Move(Direction::Down),
                KeyboardEvent::Key(Key::Named(key::Named::ArrowUp)) => Action::Move(Direction::Up),
                KeyboardEvent::Key(Key::Named(key::Named::ArrowLeft)) => Action::Move(Direction::Left),
                KeyboardEvent::Key(Key::Named(key::Named::ArrowRight)) => Action::Move(Direction::Right),
                KeyboardEvent::Key(Key::Named(key::Named::Insert)) => Action::InsertPattern,
                KeyboardEvent::Key(KEY_PLUS) => Action::NextPattern,
                KeyboardEvent::Key(KEY_MINUS) => Action::PreviousPattern,
                KeyboardEvent::Key(Key::Named(key::Named::Delete)) => Action::ClearUnit,
                KeyboardEvent::Key(Key::Named(key::Named::Space)) => Action::TogglePlay,
                KeyboardEvent::Key(KEY_MULTIPLY) => Action::ModifyDefaultOctave(1),
                KeyboardEvent::Key(KEY_DIVIDE) => Action::ModifyDefaultOctave(-1),
            ),
        );

        KeyBindings { context_bindings }
    }
}
