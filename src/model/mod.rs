use rust_utils_macro::EnumValue;

use self::value_object::OctaveValue;

pub mod pattern;

#[derive(Copy, Clone, PartialEq, Eq, Debug, ordinalizer::Ordinal)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
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
    pub const MAX_OCTAVE: u8 = 9;

    define_value_object!(pub HexDigit, u8, 0, |v| { v <= 0xF });
    define_value_object!(pub OctaveValue, u8, 5, |v| { v <= MAX_OCTAVE });
}
