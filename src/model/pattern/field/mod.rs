use crate::key_bindings::Action;

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

impl TryFrom<Action> for Note {
    type Error = ();

    fn try_from(value: Action) -> Result<Self, Self::Error> {
        match value {
            Action::NoteA => Ok(Note::A),
            Action::NoteB => Ok(Note::B),
            Action::NoteC => Ok(Note::C),
            Action::NoteD => Ok(Note::D),
            Action::NoteE => Ok(Note::E),
            Action::NoteF => Ok(Note::F),
            Action::NoteG => Ok(Note::G),
            Action::NoteCSharp => Ok(Note::CSharp),
            Action::NoteDSharp => Ok(Note::DSharp),
            Action::NoteFSharp => Ok(Note::FSharp),
            Action::NoteGSharp => Ok(Note::GSharp),
            Action::NoteASharp => Ok(Note::ASharp),
            _ => Err(()),
        }
    }
}