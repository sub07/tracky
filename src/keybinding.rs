use std::collections::HashMap;

use iced::keyboard::KeyCode;
use rust_utils::hash_map_of;

use crate::model::{Direction, HexValue, Note, OctaveValue};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Action {
    Note(Note),
    Hex(HexValue),
    Octave(OctaveValue),
    ClearUnit,
    Move(Direction),
    InsertPattern,
    NextPattern,
    PreviousPattern,
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum PatternInputType {
    Note,
    Octave,
    Hex,
    Global,
}

pub struct KeyBindings {
    pub context_bindings: HashMap<PatternInputType, HashMap<KeyCode, Action>>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let context_bindings = hash_map_of!(
            PatternInputType::Note => hash_map_of!(
                KeyCode::A => Action::Note(Note::C),
                KeyCode::Key2 => Action::Note(Note::CSharp),
                KeyCode::Z => Action::Note(Note::D),
                KeyCode::Key3 => Action::Note(Note::DSharp),
                KeyCode::E => Action::Note(Note::E),
                KeyCode::R => Action::Note(Note::F),
                KeyCode::Key5 => Action::Note(Note::FSharp),
                KeyCode::T => Action::Note(Note::G),
                KeyCode::Key6 => Action::Note(Note::GSharp),
                KeyCode::Y => Action::Note(Note::A),
                KeyCode::Key7 => Action::Note(Note::ASharp),
                KeyCode::U => Action::Note(Note::B),
                KeyCode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Octave => hash_map_of!(
                KeyCode::Key0 => Action::Octave(OctaveValue::new(0).unwrap()),
                KeyCode::Key1 => Action::Octave(OctaveValue::new(1).unwrap()),
                KeyCode::Key2 => Action::Octave(OctaveValue::new(2).unwrap()),
                KeyCode::Key3 => Action::Octave(OctaveValue::new(3).unwrap()),
                KeyCode::Key4 => Action::Octave(OctaveValue::new(4).unwrap()),
                KeyCode::Key5 => Action::Octave(OctaveValue::new(5).unwrap()),
                KeyCode::Key6 => Action::Octave(OctaveValue::new(6).unwrap()),
                KeyCode::Key7 => Action::Octave(OctaveValue::new(7).unwrap()),
                KeyCode::Key8 => Action::Octave(OctaveValue::new(8).unwrap()),
                KeyCode::Key9 => Action::Octave(OctaveValue::new(9).unwrap()),
                KeyCode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Hex => hash_map_of!(
                KeyCode::Key0 => Action::Hex(HexValue::new(0x0).unwrap()),
                KeyCode::Key1 => Action::Hex(HexValue::new(0x1).unwrap()),
                KeyCode::Key2 => Action::Hex(HexValue::new(0x2).unwrap()),
                KeyCode::Key3 => Action::Hex(HexValue::new(0x3).unwrap()),
                KeyCode::Key4 => Action::Hex(HexValue::new(0x4).unwrap()),
                KeyCode::Key5 => Action::Hex(HexValue::new(0x5).unwrap()),
                KeyCode::Key6 => Action::Hex(HexValue::new(0x6).unwrap()),
                KeyCode::Key7 => Action::Hex(HexValue::new(0x7).unwrap()),
                KeyCode::Key8 => Action::Hex(HexValue::new(0x8).unwrap()),
                KeyCode::Key9 => Action::Hex(HexValue::new(0x9).unwrap()),
                KeyCode::A => Action::Hex(HexValue::new(0xA).unwrap()),
                KeyCode::B => Action::Hex(HexValue::new(0xB).unwrap()),
                KeyCode::C => Action::Hex(HexValue::new(0xC).unwrap()),
                KeyCode::D => Action::Hex(HexValue::new(0xD).unwrap()),
                KeyCode::E => Action::Hex(HexValue::new(0xE).unwrap()),
                KeyCode::F => Action::Hex(HexValue::new(0xF).unwrap()),
                KeyCode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Global => hash_map_of!(
                KeyCode::Down => Action::Move(Direction::Down),
                KeyCode::Up => Action::Move(Direction::Up),
                KeyCode::Left => Action::Move(Direction::Left),
                KeyCode::Right => Action::Move(Direction::Right),
                KeyCode::Insert => Action::InsertPattern,
                KeyCode::Plus => Action::NextPattern,
                KeyCode::Minus => Action::PreviousPattern,
            ),
        );

        KeyBindings { context_bindings }
    }
}
