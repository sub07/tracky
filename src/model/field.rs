pub mod value_object {
    use rust_utils::{define_bounded_value_object, generate_bounded_value_object_consts};
    define_bounded_value_object! {
        pub HexDigit: u8,
        default: 0,
        min: 0,
        max: 0xF,
    }

    generate_bounded_value_object_consts! {
        HexDigit,
        HEX_0 => 0x0,
        HEX_1 => 0x1,
        HEX_2 => 0x2,
        HEX_3 => 0x3,
        HEX_4 => 0x4,
        HEX_5 => 0x5,
        HEX_6 => 0x6,
        HEX_7 => 0x7,
        HEX_8 => 0x8,
        HEX_9 => 0x9,
        HEX_A => 0xA,
        HEX_B => 0xB,
        HEX_C => 0xC,
        HEX_D => 0xD,
        HEX_E => 0xE,
        HEX_F => 0xF,
    }

    define_bounded_value_object! {
        pub OctaveValue: i32,
        default: 5,
        min: 0,
        max: 9,
    }

    generate_bounded_value_object_consts! {
        OctaveValue,
        OCTAVE_0 => 0x0,
        OCTAVE_1 => 0x1,
        OCTAVE_2 => 0x2,
        OCTAVE_3 => 0x3,
        OCTAVE_4 => 0x4,
        OCTAVE_5 => 0x5,
        OCTAVE_6 => 0x6,
        OCTAVE_7 => 0x7,
        OCTAVE_8 => 0x8,
        OCTAVE_9 => 0x9,
    }
}

pub struct Field<T>(Option<T>);

impl<T> Default for Field<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Field<T> {
    pub fn new(value: T) -> Self {
        Self(Some(value))
    }

    pub fn set(&mut self, value: T) {
        self.0 = Some(value)
    }

    pub fn clear(&mut self) {
        self.0 = None
    }

    pub fn value(&self) -> Option<&T> {
        self.0.as_ref()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, ordinalizer::Ordinal)]
pub enum NoteName {
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

pub type Note = (NoteName, value_object::OctaveValue);

pub enum NoteFieldValue {
    Note(Note),
    Cut,
}
