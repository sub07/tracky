use derive_new::new;
use sdl2::keyboard::Keycode;

use crate::key_bindings::{KeyBindings, PatternInputUnitAction};
use crate::model::field::Note;
use crate::model::patterns::PatternsContext;

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
        if value > 9 { panic!("Invalid value for an octave"); }
        OctaveValue { value }
    }

    pub fn value(&self) -> u8 { self.value }
}

#[derive(new)]
pub struct NoteValue {
    pub note: Note,
    pub octave: OctaveValue,
}

#[derive(new, Default)]
pub struct NoteField {
    pub note: Option<NoteValue>,
}

impl NoteField {
    pub fn handle_input(&mut self, key: Keycode, key_bindings: &KeyBindings, local_x_cursor: usize, patterns_context: &PatternsContext) {
        match local_x_cursor {
            0 if let Some(action) = key_bindings.note_mapping.get(&key) => {
                match action {
                    PatternInputUnitAction::NoteA |
                    PatternInputUnitAction::NoteB |
                    PatternInputUnitAction::NoteC |
                    PatternInputUnitAction::NoteD |
                    PatternInputUnitAction::NoteE |
                    PatternInputUnitAction::NoteF |
                    PatternInputUnitAction::NoteG |
                    PatternInputUnitAction::NoteCSharp |
                    PatternInputUnitAction::NoteDSharp |
                    PatternInputUnitAction::NoteFSharp |
                    PatternInputUnitAction::NoteGSharp |
                    PatternInputUnitAction::NoteASharp => {
                        let note = Note::try_from(*action).unwrap();
                        let octave = if let Some(note_value) = &self.note {
                            note_value.octave
                        } else {
                            patterns_context.default_octave
                        };

                        self.note = Some(NoteValue::new(note, octave))
                    }
                    PatternInputUnitAction::ClearUnit => {
                        self.note = None;
                    }
                    _ => {}
                }
            }
            2 if let Some(action) = key_bindings.octave_mapping.get(&key) => {
                match action {
                    PatternInputUnitAction::Octave0 |
                    PatternInputUnitAction::Octave1 |
                    PatternInputUnitAction::Octave2 |
                    PatternInputUnitAction::Octave3 |
                    PatternInputUnitAction::Octave4 |
                    PatternInputUnitAction::Octave5 |
                    PatternInputUnitAction::Octave6 |
                    PatternInputUnitAction::Octave7 |
                    PatternInputUnitAction::Octave8 |
                    PatternInputUnitAction::Octave9 => {
                        let octave_value = match action {
                            PatternInputUnitAction::Octave0 => 0u8,
                            PatternInputUnitAction::Octave1 => 1,
                            PatternInputUnitAction::Octave2 => 2,
                            PatternInputUnitAction::Octave3 => 3,
                            PatternInputUnitAction::Octave4 => 4,
                            PatternInputUnitAction::Octave5 => 5,
                            PatternInputUnitAction::Octave6 => 6,
                            PatternInputUnitAction::Octave7 => 7,
                            PatternInputUnitAction::Octave8 => 8,
                            PatternInputUnitAction::Octave9 => 9,
                            _ => panic!("Invalid action : {action:?}"),
                        };

                        if let Some(note_value) = &mut self.note {
                            note_value.octave = OctaveValue::new(octave_value);
                        }
                    },
                    PatternInputUnitAction::ClearUnit => {
                        self.note = None;
                    }
                    _ => {}
                }
            }
            1 => panic!("Cursor cannot be on alteration"),
            0 | 2 => {
                println!("Invalid key({key}) binding for this cursor position({local_x_cursor}) for note");
            }
            _ => panic!("Invalid cursor x : {local_x_cursor}"),
        }
    }
}
