use std::collections::HashMap;

use rust_utils::hash_map_of;
use winit::event::VirtualKeyCode;

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
    pub context_bindings: HashMap<PatternInputType, HashMap<VirtualKeyCode, Action>>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let context_bindings = hash_map_of!(
            PatternInputType::Note => hash_map_of!(
                VirtualKeyCode::A => Action::NoteC,
                VirtualKeyCode::Key2 => Action::NoteCSharp,
                VirtualKeyCode::Z => Action::NoteD,
                VirtualKeyCode::Key3 => Action::NoteDSharp,
                VirtualKeyCode::E => Action::NoteE,
                VirtualKeyCode::R => Action::NoteF,
                VirtualKeyCode::Key5 => Action::NoteFSharp,
                VirtualKeyCode::T => Action::NoteG,
                VirtualKeyCode::Key6 => Action::NoteGSharp,
                VirtualKeyCode::Y => Action::NoteA,
                VirtualKeyCode::Key7 => Action::NoteASharp,
                VirtualKeyCode::U => Action::NoteB,
                VirtualKeyCode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Octave => hash_map_of!(
                VirtualKeyCode::Key0 => Action::Octave0,
                VirtualKeyCode::Key1 => Action::Octave1,
                VirtualKeyCode::Key2 => Action::Octave2,
                VirtualKeyCode::Key3 => Action::Octave3,
                VirtualKeyCode::Key4 => Action::Octave4,
                VirtualKeyCode::Key5 => Action::Octave5,
                VirtualKeyCode::Key6 => Action::Octave6,
                VirtualKeyCode::Key7 => Action::Octave7,
                VirtualKeyCode::Key8 => Action::Octave8,
                VirtualKeyCode::Key9 => Action::Octave9,
                VirtualKeyCode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Hex => hash_map_of!(
                VirtualKeyCode::Key0 => Action::Hex0,
                VirtualKeyCode::Key1 => Action::Hex1,
                VirtualKeyCode::Key2 => Action::Hex2,
                VirtualKeyCode::Key3 => Action::Hex3,
                VirtualKeyCode::Key4 => Action::Hex4,
                VirtualKeyCode::Key5 => Action::Hex5,
                VirtualKeyCode::Key6 => Action::Hex6,
                VirtualKeyCode::Key7 => Action::Hex7,
                VirtualKeyCode::Key8 => Action::Hex8,
                VirtualKeyCode::Key9 => Action::Hex9,
                VirtualKeyCode::A => Action::HexA,
                VirtualKeyCode::B => Action::HexB,
                VirtualKeyCode::C => Action::HexC,
                VirtualKeyCode::D => Action::HexD,
                VirtualKeyCode::E => Action::HexE,
                VirtualKeyCode::F => Action::HexF,
                VirtualKeyCode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Global => hash_map_of!(
                VirtualKeyCode::Down => Action::MoveDown,
                VirtualKeyCode::Up => Action::MoveUp,
                VirtualKeyCode::Left => Action::MoveLeft,
                VirtualKeyCode::Right => Action::MoveRight,
                VirtualKeyCode::Insert => Action::InsertPattern,
                VirtualKeyCode::Plus => Action::NextPattern,
                VirtualKeyCode::Minus => Action::PreviousPattern,
            ),
        );

        KeyBindings { context_bindings }
    }
}
