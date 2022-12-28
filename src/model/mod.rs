pub mod pattern_line;
pub mod pattern;
pub mod patterns;
pub mod field;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn vec(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

// trace_macros!(true);
macro_rules! count_enum_variant {
    () => (0usize);
    ($variant:ident $($variants:ident)*) => (1usize + count_enum_variant!($($variants)*));
}

macro_rules! pattern_line_element_auto {
    (
        pub enum $enum_name:ident {
            $(
                $variants:ident,
            )*
        }
    ) => {
        pub enum $enum_name {
            $(
                $variants,
            )*
        }

        impl $enum_name {
            const ENUM_ARRAY: [$enum_name; count_enum_variant!($($variants )*)] = [$($enum_name::$variants,)*];
        }
    }
}


pattern_line_element_auto! {
    pub enum PatternLineElement {
        Note,
        Velocity,
    }
}

impl PatternLineElement {
    pub const i: usize = PatternLineElement::line_len();
    const PATTERN_LINE_ELEMENT_ARRAY: [PatternLineElement; 2] = [PatternLineElement::Note, PatternLineElement::Velocity];

    pub const fn line_len() -> usize {
        count_enum_variant!(Velocity Note);

        let mut sum = 0;
        let mut i = 0;
        while i < PATTERN_LINE_ELEMENT_ARRAY.len() {
            sum += PATTERN_LINE_ELEMENT_ARRAY[i].len();
            i += 1;
        }
        sum
    }

    pub const fn len(&self) -> usize {
        match self {
            PatternLineElement::Note => 3,
            PatternLineElement::Velocity => 2,
        }
    }
}

const PATTERN_LINE_ELEMENT_ARRAY: [PatternLineElement; 2] = [PatternLineElement::Note, PatternLineElement::Velocity];
pub const PATTERN_LINE_LEN: usize = PatternLineElement::line_len();

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
    Empty,
}
