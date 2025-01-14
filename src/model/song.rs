use super::{
    pattern::{HexDigit, NoteName, OctaveValue, Patterns},
    Direction,
};

pub struct State {
    pub patterns: Patterns,
    pub global_octave: OctaveValue,
    pub line_per_second: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            patterns: Default::default(),
            global_octave: Default::default(),
            line_per_second: 16.0,
        }
    }
}

#[derive(Debug)]
pub enum Event {
    MutateGlobalOctave {
        increment: i32,
    },
    SetNoteField {
        note: NoteName,
        octave_modifier: i32, // -1 for global octave - 1, when hitting different keyboard row for example
    },
    MoveCursor(Direction),
    SetNoteFieldToCut,
    ClearField,
    SetOctaveField(OctaveValue),
    SetHexField(HexDigit),
    NewPattern,
    NextPattern,
    PreviousPattern,
}
