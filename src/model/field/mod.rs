use crate::key_bindings::PatternInputUnitAction;

pub mod note;
pub mod velocity;

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
}

impl TryFrom<PatternInputUnitAction> for Note {
    type Error = ();

    fn try_from(value: PatternInputUnitAction) -> Result<Self, Self::Error> {
        match value {
            PatternInputUnitAction::NoteA => Ok(Note::A),
            PatternInputUnitAction::NoteB => Ok(Note::B),
            PatternInputUnitAction::NoteC => Ok(Note::C),
            PatternInputUnitAction::NoteD => Ok(Note::D),
            PatternInputUnitAction::NoteE => Ok(Note::E),
            PatternInputUnitAction::NoteF => Ok(Note::F),
            PatternInputUnitAction::NoteG => Ok(Note::G),
            PatternInputUnitAction::NoteCSharp => Ok(Note::CSharp),
            PatternInputUnitAction::NoteDSharp => Ok(Note::DSharp),
            PatternInputUnitAction::NoteFSharp => Ok(Note::FSharp),
            PatternInputUnitAction::NoteGSharp => Ok(Note::GSharp),
            PatternInputUnitAction::NoteASharp => Ok(Note::ASharp),
            _ => Err(()),
        }
    }
}