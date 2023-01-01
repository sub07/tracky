pub mod column_line;
pub mod column;
pub mod pattern;
pub mod field;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
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
            pub const LINE_LEN: usize = $enum_name::line_len();
            pub const NB_VARIANT: usize = $enum_name::nb_elem();


            pub const fn len(&self) -> usize {
                match self {
                    $(
                        $enum_name::$variants => $len,
                    )*
                }
            }

            pub const fn nb_elem() -> usize {
                $enum_name::ENUM_ARRAY.len()
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
