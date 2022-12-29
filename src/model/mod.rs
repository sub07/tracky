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

macro_rules! count_enum_variant {
    () => (0usize);
    ($variant:ident $($variants:ident)*) => (1usize + count_enum_variant!($($variants)*));
}

macro_rules! pattern_line_element_auto {
    (
        pub enum $enum_name:ident {
            $(
                $variants:ident => $len:literal,
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

            const fn len(&self) -> usize {
                match self {
                    $(
                        $enum_name::$variants => $len,
                    )*
                }
            }

            const fn line_len() -> usize {
                let mut sum = 0;
                let mut i = 0;
                while i < PatternLineElement::ENUM_ARRAY.len() {
                    sum += PatternLineElement::ENUM_ARRAY[i].len();
                    i += 1;
                }
                sum
            }
        }
    }
}


pattern_line_element_auto! {
    pub enum PatternLineElement {
        Note => 3,
        Velocity => 2,
    }
}

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
