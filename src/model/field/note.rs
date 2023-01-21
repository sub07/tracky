use rust_utils_macro::New;

use crate::model::field::Note;

#[derive(Copy, Clone)]
pub struct OctaveValue {
    value: u8,
}

impl Default for OctaveValue {
    fn default() -> Self {
        OctaveValue::new(5)
    }
}

impl OctaveValue {
    pub fn new(value: u8) -> OctaveValue {
        if value > 9 { panic!("Invalid value for an octave : {value}"); }
        OctaveValue { value }
    }

    pub fn value(&self) -> u8 { self.value }
}

#[derive(New)]
pub struct NoteValue {
    pub note: Note,
    pub octave: OctaveValue,
}

#[derive(New, Default)]
pub struct NoteField {
    pub note: Option<NoteValue>,
}

impl NoteField {
    pub fn set_octave(&mut self, octave: OctaveValue) {
        if let Some(note) = &mut self.note {
            note.octave = octave;
        }
    }

    pub fn set_note(&mut self, note: Note, default_octave: OctaveValue) {
        let octave = if let Some(NoteValue { octave, .. }) = &self.note {
            *octave
        } else {
            default_octave
        };
        self.note = Some(NoteValue {
            note,
            octave,
        })
    }

    pub fn clear(&mut self) {
        self.note = None;
    }
}
