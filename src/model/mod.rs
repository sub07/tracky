use crate::define_value_object;

pub mod pattern;
pub mod song_info;
pub mod sample;

define_value_object!(pub Bpm, f32, 120.0, |v| { (1.0..=999.0).contains(&v) });
