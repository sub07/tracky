pub mod value_object {
    use rust_utils::define_value_object;
    pub const MAX_OCTAVE: u8 = 9;

    define_value_object!(pub HexDigit, u8, 0, |v| { v <= 0xF });
    define_value_object!(pub OctaveValue, u8, 5, |v| { v <= MAX_OCTAVE });
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
