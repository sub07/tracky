use std::collections::HashMap;

use iced::keyboard::KeyCode;
use rust_utils::hash_map_of;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Action {
    NoteA,
    NoteB,
    NoteC,
    NoteD,
    NoteE,
    NoteF,
    NoteG,
    NoteCSharp,
    NoteDSharp,
    NoteFSharp,
    NoteGSharp,
    NoteASharp,

    Hex0,
    Hex1,
    Hex2,
    Hex3,
    Hex4,
    Hex5,
    Hex6,
    Hex7,
    Hex8,
    Hex9,
    HexA,
    HexB,
    HexC,
    HexD,
    HexE,
    HexF,

    Octave0,
    Octave1,
    Octave2,
    Octave3,
    Octave4,
    Octave5,
    Octave6,
    Octave7,
    Octave8,
    Octave9,

    ClearUnit,

    MoveDown,
    MoveUp,
    MoveLeft,
    MoveRight,
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
                KeyCode::A => Action::NoteC,
                KeyCode::Key2 => Action::NoteCSharp,
                KeyCode::Z => Action::NoteD,
                KeyCode::Key3 => Action::NoteDSharp,
                KeyCode::E => Action::NoteE,
                KeyCode::R => Action::NoteF,
                KeyCode::Key5 => Action::NoteFSharp,
                KeyCode::T => Action::NoteG,
                KeyCode::Key6 => Action::NoteGSharp,
                KeyCode::Y => Action::NoteA,
                KeyCode::Key7 => Action::NoteASharp,
                KeyCode::U => Action::NoteB,
                KeyCode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Octave => hash_map_of!(
                KeyCode::Key0 => Action::Octave0,
                KeyCode::Key1 => Action::Octave1,
                KeyCode::Key2 => Action::Octave2,
                KeyCode::Key3 => Action::Octave3,
                KeyCode::Key4 => Action::Octave4,
                KeyCode::Key5 => Action::Octave5,
                KeyCode::Key6 => Action::Octave6,
                KeyCode::Key7 => Action::Octave7,
                KeyCode::Key8 => Action::Octave8,
                KeyCode::Key9 => Action::Octave9,
                KeyCode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Hex => hash_map_of!(
                KeyCode::Key0 => Action::Hex0,
                KeyCode::Key1 => Action::Hex1,
                KeyCode::Key2 => Action::Hex2,
                KeyCode::Key3 => Action::Hex3,
                KeyCode::Key4 => Action::Hex4,
                KeyCode::Key5 => Action::Hex5,
                KeyCode::Key6 => Action::Hex6,
                KeyCode::Key7 => Action::Hex7,
                KeyCode::Key8 => Action::Hex8,
                KeyCode::Key9 => Action::Hex9,
                KeyCode::A => Action::HexA,
                KeyCode::B => Action::HexB,
                KeyCode::C => Action::HexC,
                KeyCode::D => Action::HexD,
                KeyCode::E => Action::HexE,
                KeyCode::F => Action::HexF,
                KeyCode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Global => hash_map_of!(
                KeyCode::Down => Action::MoveDown,
                KeyCode::Up => Action::MoveUp,
                KeyCode::Left => Action::MoveLeft,
                KeyCode::Right => Action::MoveRight,
                KeyCode::Insert => Action::InsertPattern,
                KeyCode::Plus => Action::NextPattern,
                KeyCode::Minus => Action::PreviousPattern,
            ),
        );

        KeyBindings { context_bindings }
    }
}
