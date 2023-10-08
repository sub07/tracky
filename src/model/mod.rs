use rust_utils_macro::EnumValue;

// pub mod pattern;
pub mod audio_channel;
pub mod field;
pub mod pattern;

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
