use anyhow::bail;
use anyhow::Result;
use rust_utils_macro::New;

pub mod pattern;

#[derive(Copy, Clone)]
pub enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    CSharp,
    DSharp,
    FSharp,
    GSharp,
    ASharp,
}

#[derive(Copy, Clone, Default)]
pub struct OctaveValue(u8);

impl OctaveValue {
    pub fn new(value: u8) -> Result<OctaveValue> {
        if value > 9 {
            bail!("Invalid value for an octave : {value}")
        } else {
            Ok(OctaveValue(value))
        }
    }
}

#[derive(New, Copy, Clone)]
pub struct NoteValue {
    pub note: Note,
    pub octave: OctaveValue,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(New, Default, Copy, Clone)]
pub struct NoteField {
    pub note: Option<NoteValue>,
}

#[derive(Default, Copy, Clone)]
pub struct HexValue(u8);

impl HexValue {
    pub fn new(value: u8) -> Result<HexValue> {
        if value > 0xF {
            bail!("Invalid value for an octave : {value}")
        }
        Ok(HexValue(value))
    }
}
