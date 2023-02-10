use std::collections::HashMap;

use rust_utils::hash_map_of;
use sdl2::keyboard::Keycode;

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
    pub context_bindings: HashMap<PatternInputType, HashMap<Keycode, Action>>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let context_bindings = hash_map_of!(
            PatternInputType::Note => hash_map_of!(
                Keycode::A => Action::NoteC,
                Keycode::Num2 => Action::NoteCSharp,
                Keycode::Z => Action::NoteD,
                Keycode::Num3 => Action::NoteDSharp,
                Keycode::E => Action::NoteE,
                Keycode::R => Action::NoteF,
                Keycode::Num5 => Action::NoteFSharp,
                Keycode::T => Action::NoteG,
                Keycode::Num6 => Action::NoteGSharp,
                Keycode::Y => Action::NoteA,
                Keycode::Num7 => Action::NoteASharp,
                Keycode::U => Action::NoteB,
                Keycode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Octave => hash_map_of!(
                Keycode::Num0 => Action::Octave0,
                Keycode::Num1 => Action::Octave1,
                Keycode::Num2 => Action::Octave2,
                Keycode::Num3 => Action::Octave3,
                Keycode::Num4 => Action::Octave4,
                Keycode::Num5 => Action::Octave5,
                Keycode::Num6 => Action::Octave6,
                Keycode::Num7 => Action::Octave7,
                Keycode::Num8 => Action::Octave8,
                Keycode::Num9 => Action::Octave9,
                Keycode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Hex => hash_map_of!(
                Keycode::Num0 => Action::Hex0,
                Keycode::Num1 => Action::Hex1,
                Keycode::Num2 => Action::Hex2,
                Keycode::Num3 => Action::Hex3,
                Keycode::Num4 => Action::Hex4,
                Keycode::Num5 => Action::Hex5,
                Keycode::Num6 => Action::Hex6,
                Keycode::Num7 => Action::Hex7,
                Keycode::Num8 => Action::Hex8,
                Keycode::Num9 => Action::Hex9,
                Keycode::A => Action::HexA,
                Keycode::B => Action::HexB,
                Keycode::C => Action::HexC,
                Keycode::D => Action::HexD,
                Keycode::E => Action::HexE,
                Keycode::F => Action::HexF,
                Keycode::Delete => Action::ClearUnit,
            ),
            PatternInputType::Global => hash_map_of!(
                Keycode::Down => Action::MoveDown,
                Keycode::Up => Action::MoveUp,
                Keycode::Left => Action::MoveLeft,
                Keycode::Right => Action::MoveRight,
                Keycode::Insert => Action::InsertPattern,
                Keycode::KpPlus => Action::NextPattern,
                Keycode::KpMinus => Action::PreviousPattern,
            ),
        );

        KeyBindings { context_bindings }
    }
}
