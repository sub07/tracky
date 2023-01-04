use std::collections::HashMap;

use sdl2::keyboard::Keycode;

#[derive(Eq, PartialEq, Hash)]
pub enum PatternValueAction {
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
}

pub struct KeyBindings {
    pub note_mapping: HashMap<Keycode, PatternValueAction>,
    pub hex_mapping: HashMap<Keycode, PatternValueAction>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        const PATTERN_VALUE_ACTION_LEN: usize = std::mem::variant_count::<PatternValueAction>();
        let note_mapping = {
            let mut map = HashMap::<Keycode, PatternValueAction>::with_capacity(PATTERN_VALUE_ACTION_LEN);
            map.insert(Keycode::Q, PatternValueAction::NoteC);
            map.insert(Keycode::Num2, PatternValueAction::NoteCSharp);
            map.insert(Keycode::W, PatternValueAction::NoteD);
            map.insert(Keycode::Num3, PatternValueAction::NoteDSharp);
            map.insert(Keycode::E, PatternValueAction::NoteE);
            map.insert(Keycode::R, PatternValueAction::NoteF);
            map.insert(Keycode::Num5, PatternValueAction::NoteFSharp);
            map.insert(Keycode::T, PatternValueAction::NoteG);
            map.insert(Keycode::Num6, PatternValueAction::NoteGSharp);
            map.insert(Keycode::Y, PatternValueAction::NoteA);
            map.insert(Keycode::Num7, PatternValueAction::NoteASharp);
            map.insert(Keycode::U, PatternValueAction::NoteB);
            map
        };


        let hex_mapping = {
            let mut map = HashMap::<Keycode, PatternValueAction>::with_capacity(PATTERN_VALUE_ACTION_LEN);
            map.insert(Keycode::Num0, PatternValueAction::Hex0);
            map.insert(Keycode::Num1, PatternValueAction::Hex1);
            map.insert(Keycode::Num2, PatternValueAction::Hex2);
            map.insert(Keycode::Num3, PatternValueAction::Hex3);
            map.insert(Keycode::Num4, PatternValueAction::Hex4);
            map.insert(Keycode::Num5, PatternValueAction::Hex5);
            map.insert(Keycode::Num6, PatternValueAction::Hex6);
            map.insert(Keycode::Num7, PatternValueAction::Hex7);
            map.insert(Keycode::Num8, PatternValueAction::Hex8);
            map.insert(Keycode::Num9, PatternValueAction::Hex9);
            map.insert(Keycode::A, PatternValueAction::HexA);
            map.insert(Keycode::B, PatternValueAction::HexB);
            map.insert(Keycode::C, PatternValueAction::HexC);
            map.insert(Keycode::D, PatternValueAction::HexD);
            map.insert(Keycode::E, PatternValueAction::HexE);
            map.insert(Keycode::F, PatternValueAction::HexF);
            map
        };

        KeyBindings {
            note_mapping,
            hex_mapping,
        }
    }
}