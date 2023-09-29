use anyhow::bail;
use anyhow::Result;
use rust_utils_macro::EnumValue;

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

#[derive(Copy, Clone)]
pub enum NoteValue {
    Note(Note, OctaveValue),
    Cut,
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

pub mod value_object {
    use rust_utils::define_value_object;

    define_value_object!(pub HexDigit, u8, 0, |v| { v <= 0xF });
}
