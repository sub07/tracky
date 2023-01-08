pub mod column_line;
pub mod column;
pub mod pattern;
pub mod field;
pub mod patterns;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

macro_rules! pattern_line_element_auto {
    (
        pub enum $enum_name:ident {
            $(
                $variants:ident => $len:literal,
            )*
        }
    ) => {
        #[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
        pub enum $enum_name {
            $(
                $variants,
            )*
        }

        impl $enum_name {
            const ENUM_ARRAY: [$enum_name; std::mem::variant_count::<$enum_name>()] = [$($enum_name::$variants,)*];
            pub const LINE_LEN: usize = $enum_name::line_len();
            pub const NB_VARIANT: usize = std::mem::variant_count::<$enum_name>();

            pub const fn len(&self) -> usize {
                match self {
                    $(
                        $enum_name::$variants => $len,
                    )*
                }
            }

            const fn line_len() -> usize {
                let mut sum = 0;
                let mut i = 0;
                while i < $enum_name::ENUM_ARRAY.len() {
                    sum += $enum_name::ENUM_ARRAY[i].len();
                    i += 1;
                }
                sum
            }
        }
    }
}


pattern_line_element_auto! {
    pub enum ColumnLineElement {
        Note => 3,
        Velocity => 2,
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum PatternInputType {
    Note,
    Octave,
    Hex,
}
