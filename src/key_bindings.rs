use std::collections::HashMap;

use rust_utils::hash_map_of;
use sdl2::keyboard::Keycode;

use crate::model::pattern::PatternInputType;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PatternInputUnitAction {
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
}

pub struct KeyBindings {
    pub context_bindings: HashMap<PatternInputType, HashMap<Keycode, PatternInputUnitAction>>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let context_bindings = hash_map_of!(
            PatternInputType::Note => hash_map_of!(
                Keycode::A => PatternInputUnitAction::NoteC,
                Keycode::Num2 => PatternInputUnitAction::NoteCSharp,
                Keycode::Z => PatternInputUnitAction::NoteD,
                Keycode::Num3 => PatternInputUnitAction::NoteDSharp,
                Keycode::E => PatternInputUnitAction::NoteE,
                Keycode::R => PatternInputUnitAction::NoteF,
                Keycode::Num5 => PatternInputUnitAction::NoteFSharp,
                Keycode::T => PatternInputUnitAction::NoteG,
                Keycode::Num6 => PatternInputUnitAction::NoteGSharp,
                Keycode::Y => PatternInputUnitAction::NoteA,
                Keycode::Num7 => PatternInputUnitAction::NoteASharp,
                Keycode::U => PatternInputUnitAction::NoteB,
                Keycode::Delete => PatternInputUnitAction::ClearUnit,
            ),
            PatternInputType::Octave => hash_map_of!(
                Keycode::Num0 => PatternInputUnitAction::Octave0,
                Keycode::Num1 => PatternInputUnitAction::Octave1,
                Keycode::Num2 => PatternInputUnitAction::Octave2,
                Keycode::Num3 => PatternInputUnitAction::Octave3,
                Keycode::Num4 => PatternInputUnitAction::Octave4,
                Keycode::Num5 => PatternInputUnitAction::Octave5,
                Keycode::Num6 => PatternInputUnitAction::Octave6,
                Keycode::Num7 => PatternInputUnitAction::Octave7,
                Keycode::Num8 => PatternInputUnitAction::Octave8,
                Keycode::Num9 => PatternInputUnitAction::Octave9,
                Keycode::Delete => PatternInputUnitAction::ClearUnit,
            ),
            PatternInputType::Hex => hash_map_of!(
                Keycode::Num0 => PatternInputUnitAction::Hex0,
                Keycode::Num1 => PatternInputUnitAction::Hex1,
                Keycode::Num2 => PatternInputUnitAction::Hex2,
                Keycode::Num3 => PatternInputUnitAction::Hex3,
                Keycode::Num4 => PatternInputUnitAction::Hex4,
                Keycode::Num5 => PatternInputUnitAction::Hex5,
                Keycode::Num6 => PatternInputUnitAction::Hex6,
                Keycode::Num7 => PatternInputUnitAction::Hex7,
                Keycode::Num8 => PatternInputUnitAction::Hex8,
                Keycode::Num9 => PatternInputUnitAction::Hex9,
                Keycode::A => PatternInputUnitAction::HexA,
                Keycode::B => PatternInputUnitAction::HexB,
                Keycode::C => PatternInputUnitAction::HexC,
                Keycode::D => PatternInputUnitAction::HexD,
                Keycode::E => PatternInputUnitAction::HexE,
                Keycode::F => PatternInputUnitAction::HexF,
                Keycode::Delete => PatternInputUnitAction::ClearUnit,
            ),
        );

        KeyBindings {
            context_bindings
        }
    }
}
