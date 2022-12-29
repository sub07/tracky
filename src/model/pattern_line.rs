use derive_new::new;
use sdl2::sys::ssize_t;
use crate::model::field::note::NoteField;

use crate::model::field::velocity::VelocityField;

#[derive(new, Default)]
pub struct PatternLine {
    pub note: NoteField,
    pub velocity: VelocityField,
}
