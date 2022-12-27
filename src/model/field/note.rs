use derive_new::new;

use crate::model::field::input_unit::InputUnit;

#[derive(new, Default)]
pub struct NoteField {
    pub note1: InputUnit,
    pub note2: InputUnit,
    pub octave: InputUnit,
}
