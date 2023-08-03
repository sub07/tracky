use anyhow::bail;
use anyhow::Result;
use rust_utils_macro::EnumValue;
use rust_utils_macro::New;

pub mod pattern;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct OctaveValue(pub u8);

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

#[derive(Eq, PartialEq, Copy, Clone, Debug, EnumValue)]
pub enum Direction {
    #[value(x: i32 = -1, y: i32 = 0)]
    Left,
    #[value(x: i32 = 1, y: i32 = 0)]
    Right,
    #[value(x: i32 = 0, y: i32 = -1)]
    Up,
    #[value(x: i32 = 0, y: i32 = 1)]
    Down,
}

#[derive(New, Default, Copy, Clone)]
pub struct NoteField {
    pub note: Option<NoteValue>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct HexValue(pub u8);

impl HexValue {
    pub fn new(value: u8) -> Result<HexValue> {
        if value > 0xF {
            bail!("Invalid value for an octave : {value}")
        }
        Ok(HexValue(value))
    }
}
