pub mod value_object {
    use std::ops::RangeInclusive;

    use rust_utils::define_value_object;
    pub const OCTAVE_VALID_RANGE: RangeInclusive<u8> = 0..=9;

    define_value_object!(pub HexDigit, u8, 0, |v| { v <= 0xF });
    define_value_object!(pub OctaveValue, u8, 5, |v| { OCTAVE_VALID_RANGE.contains(&v) });
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
