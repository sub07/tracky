use std::collections::HashMap;

use sdl2::keyboard::Keycode;

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
    pub note_mapping: HashMap<Keycode, PatternInputUnitAction>,
    pub octave_mapping: HashMap<Keycode, PatternInputUnitAction>,
    pub hex_mapping: HashMap<Keycode, PatternInputUnitAction>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        const PATTERN_VALUE_ACTION_LEN: usize = std::mem::variant_count::<PatternInputUnitAction>();
        let note_mapping = {
            let mut map = HashMap::<Keycode, PatternInputUnitAction>::with_capacity(PATTERN_VALUE_ACTION_LEN);
            map.insert(Keycode::A, PatternInputUnitAction::NoteC);
            map.insert(Keycode::Num2, PatternInputUnitAction::NoteCSharp);
            map.insert(Keycode::Z, PatternInputUnitAction::NoteD);
            map.insert(Keycode::Num3, PatternInputUnitAction::NoteDSharp);
            map.insert(Keycode::E, PatternInputUnitAction::NoteE);
            map.insert(Keycode::R, PatternInputUnitAction::NoteF);
            map.insert(Keycode::Num5, PatternInputUnitAction::NoteFSharp);
            map.insert(Keycode::T, PatternInputUnitAction::NoteG);
            map.insert(Keycode::Num6, PatternInputUnitAction::NoteGSharp);
            map.insert(Keycode::Y, PatternInputUnitAction::NoteA);
            map.insert(Keycode::Num7, PatternInputUnitAction::NoteASharp);
            map.insert(Keycode::U, PatternInputUnitAction::NoteB);
            map.insert(Keycode::Delete, PatternInputUnitAction::ClearUnit);
            map
        };

        let octave_mapping = {
            let mut map = HashMap::<Keycode, PatternInputUnitAction>::with_capacity(PATTERN_VALUE_ACTION_LEN);
            map.insert(Keycode::Num0, PatternInputUnitAction::Octave0);
            map.insert(Keycode::Num1, PatternInputUnitAction::Octave1);
            map.insert(Keycode::Num2, PatternInputUnitAction::Octave2);
            map.insert(Keycode::Num3, PatternInputUnitAction::Octave3);
            map.insert(Keycode::Num4, PatternInputUnitAction::Octave4);
            map.insert(Keycode::Num5, PatternInputUnitAction::Octave5);
            map.insert(Keycode::Num6, PatternInputUnitAction::Octave6);
            map.insert(Keycode::Num7, PatternInputUnitAction::Octave7);
            map.insert(Keycode::Num8, PatternInputUnitAction::Octave8);
            map.insert(Keycode::Num9, PatternInputUnitAction::Octave9);
            map.insert(Keycode::Delete, PatternInputUnitAction::ClearUnit);
            map
        };


        let hex_mapping = {
            let mut map = HashMap::<Keycode, PatternInputUnitAction>::with_capacity(PATTERN_VALUE_ACTION_LEN);
            map.insert(Keycode::Num0, PatternInputUnitAction::Hex0);
            map.insert(Keycode::Num1, PatternInputUnitAction::Hex1);
            map.insert(Keycode::Num2, PatternInputUnitAction::Hex2);
            map.insert(Keycode::Num3, PatternInputUnitAction::Hex3);
            map.insert(Keycode::Num4, PatternInputUnitAction::Hex4);
            map.insert(Keycode::Num5, PatternInputUnitAction::Hex5);
            map.insert(Keycode::Num6, PatternInputUnitAction::Hex6);
            map.insert(Keycode::Num7, PatternInputUnitAction::Hex7);
            map.insert(Keycode::Num8, PatternInputUnitAction::Hex8);
            map.insert(Keycode::Num9, PatternInputUnitAction::Hex9);
            map.insert(Keycode::A, PatternInputUnitAction::HexA);
            map.insert(Keycode::B, PatternInputUnitAction::HexB);
            map.insert(Keycode::C, PatternInputUnitAction::HexC);
            map.insert(Keycode::D, PatternInputUnitAction::HexD);
            map.insert(Keycode::E, PatternInputUnitAction::HexE);
            map.insert(Keycode::F, PatternInputUnitAction::HexF);
            map.insert(Keycode::Delete, PatternInputUnitAction::ClearUnit);
            map
        };

        KeyBindings {
            note_mapping,
            octave_mapping,
            hex_mapping,
        }
    }
}