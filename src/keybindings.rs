use std::{collections::HashMap, hash::Hash};

use bimap::BiMap;
use joy_collection_utils::hash_map_of;
use winit::keyboard::{KeyCode, ModifiersState};

use crate::{
    event::{self, Action},
    model::pattern::{HexDigit, NoteName, OctaveValue},
    utils::Direction,
    view::screen::song_editor,
};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct InputContext(
    pub InputLocation,
    pub InputType,
    pub ModifiersState,
    pub KeyCode,
);

impl From<KeyCode> for InputContext {
    fn from(key: KeyCode) -> Self {
        InputContext(
            InputLocation::Global,
            InputType::Normal,
            ModifiersState::empty(),
            key,
        )
    }
}

impl From<(ModifiersState, KeyCode)> for InputContext {
    fn from((modifiers, key): (ModifiersState, KeyCode)) -> Self {
        InputContext(InputLocation::Global, InputType::Normal, modifiers, key)
    }
}

impl From<(InputType, ModifiersState, KeyCode)> for InputContext {
    fn from((input_type, modifiers, key): (InputType, ModifiersState, KeyCode)) -> Self {
        InputContext(InputLocation::Global, input_type, modifiers, key)
    }
}

impl From<(InputLocation, InputType, ModifiersState, KeyCode)> for InputContext {
    fn from(
        (context, input_type, modifiers, key): (InputLocation, InputType, ModifiersState, KeyCode),
    ) -> Self {
        InputContext(context, input_type, modifiers, key)
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum InputLocation {
    Global,
    SongEditor,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum InputType {
    Note,
    Octave,
    Hex,
    Normal,
    Text,
}

#[derive(PartialEq, Eq, Hash)]
struct Keybind(ModifiersState, KeyCode);

impl From<KeyCode> for Keybind {
    fn from(key: KeyCode) -> Self {
        Keybind(ModifiersState::empty(), key)
    }
}

impl From<(ModifiersState, KeyCode)> for Keybind {
    fn from((modifiers, key): (ModifiersState, KeyCode)) -> Self {
        Keybind(modifiers, key)
    }
}

pub struct Keybindings {
    bindings: BiMap<InputContext, Action>,
}

impl Keybindings {
    pub fn action(&self, input_context: InputContext) -> Option<Action> {
        self.bindings.get_by_left(&input_context).cloned()
    }
}

fn song_note_event(n: NoteName, o: i32) -> Action {
    Action::SongScreen(song_editor::Action::SetNoteField {
        note: n,
        octave_modifier: o,
    })
}

#[macro_export]
macro_rules! bi_hash_map_of {
    () => {
        bimap::BiMap::new()
    };
    ($($key:expr => $value:expr),* $(,)?) => {
        bimap::BiMap::from_iter([
            $(
                ($key.into(), $value.into()),
            )*
        ])
    };
}

impl Default for Keybindings {
    fn default() -> Self {
        // let context_bindings = hash_map_of!(
        //     InputContext::Note => hash_map_of!(
        //         KeyCode::KeyQ => song_note_event(NoteName::C, 0),
        //         KeyCode::Digit2 => song_note_event(NoteName::CSharp, 0),
        //         KeyCode::KeyW => song_note_event(NoteName::D, 0),
        //         KeyCode::Digit3 => song_note_event(NoteName::DSharp, 0),
        //         KeyCode::KeyE => song_note_event(NoteName::E, 0),
        //         KeyCode::KeyR => song_note_event(NoteName::F, 0),
        //         KeyCode::Digit5 => song_note_event(NoteName::FSharp, 0),
        //         KeyCode::KeyT => song_note_event(NoteName::G, 0),
        //         KeyCode::Digit6 => song_note_event(NoteName::GSharp, 0),
        //         KeyCode::KeyY => song_note_event(NoteName::A, 0),
        //         KeyCode::Digit7 => song_note_event(NoteName::ASharp, 0),
        //         KeyCode::KeyU => song_note_event(NoteName::B, 0),
        //         KeyCode::Digit1 => Action::SetNoteCut,
        //     ),
        //     InputContext::Octave => hash_map_of!(
        //         KeyCode::Digit0 => Action::SetOctaveField(OctaveValue::OCTAVE_0),
        //         KeyCode::Digit1 => Action::SetOctaveField(OctaveValue::OCTAVE_1),
        //         KeyCode::Digit2 => Action::SetOctaveField(OctaveValue::OCTAVE_2),
        //         KeyCode::Digit3 => Action::SetOctaveField(OctaveValue::OCTAVE_3),
        //         KeyCode::Digit4 => Action::SetOctaveField(OctaveValue::OCTAVE_4),
        //         KeyCode::Digit5 => Action::SetOctaveField(OctaveValue::OCTAVE_5),
        //         KeyCode::Digit6 => Action::SetOctaveField(OctaveValue::OCTAVE_6),
        //         KeyCode::Digit7 => Action::SetOctaveField(OctaveValue::OCTAVE_7),
        //         KeyCode::Digit8 => Action::SetOctaveField(OctaveValue::OCTAVE_8),
        //         KeyCode::Digit9 => Action::SetOctaveField(OctaveValue::OCTAVE_9),
        //     ),
        //     InputContext::Hex => hash_map_of!(
        //         KeyCode::Digit0 => Action::SetHexField(HexDigit::HEX_0),
        //         KeyCode::Digit1 => Action::SetHexField(HexDigit::HEX_1),
        //         KeyCode::Digit2 => Action::SetHexField(HexDigit::HEX_2),
        //         KeyCode::Digit3 => Action::SetHexField(HexDigit::HEX_3),
        //         KeyCode::Digit4 => Action::SetHexField(HexDigit::HEX_4),
        //         KeyCode::Digit5 => Action::SetHexField(HexDigit::HEX_5),
        //         KeyCode::Digit6 => Action::SetHexField(HexDigit::HEX_6),
        //         KeyCode::Digit7 => Action::SetHexField(HexDigit::HEX_7),
        //         KeyCode::Digit8 => Action::SetHexField(HexDigit::HEX_8),
        //         KeyCode::Digit9 => Action::SetHexField(HexDigit::HEX_9),
        //     ),
        //     InputContext::Global => hash_map_of!(
        //         KeyCode::ArrowDown => Action::Move(Direction::Down),
        //         KeyCode::ArrowUp => Action::Move(Direction::Up),
        //         KeyCode::ArrowLeft => Action::Move(Direction::Left),
        //         KeyCode::ArrowRight => Action::Move(Direction::Right),
        //         KeyCode::Insert => Action::CreateNewPattern,
        //         KeyCode::NumpadAdd => Action::GoToNextPattern,
        //         KeyCode::NumpadSubtract => Action::GoToPreviousPattern,
        //         KeyCode::Delete => Action::ClearField,
        //         KeyCode::Space => Action::TogglePlay,
        //         KeyCode::NumpadMultiply => Action::ChangeGlobalOctave { increment: 1 },
        //         KeyCode::Tab => Action::Forward,
        //         (ModifiersState::SHIFT, KeyCode::Tab) => Action::Backward,
        //         KeyCode::NumpadDivide => Action::ChangeGlobalOctave { increment: -1 },
        //         KeyCode::Escape => Action::Cancel,
        //         KeyCode::Enter => Action::Confirm,
        //         KeyCode::F1 => Action::RequestChangeScreenToDeviceSelection,
        //         KeyCode::F2 => Action::RequestChangeScreenToSongEditor,
        //         KeyCode::F11 => Action::ToggleFullscreen,
        //         KeyCode::F8 => Action::KillNotes,
        //         (ModifiersState::ALT, KeyCode::KeyV) => Action::ShowGlobalVolumePopup,
        //         KeyCode::PageDown => Action::ChangeSelectedInstrument { increment: 1 },
        //         KeyCode::PageUp => Action::ChangeSelectedInstrument { increment: -1 },
        //     ),
        //     InputContext::Text => hash_map_of!(
        //         KeyCode::Backspace => Action::Text(event::Text::RemoveCharAtCursor),
        //         KeyCode::ArrowLeft => Action::Text(event::Text::MoveCursorLeft),
        //         KeyCode::ArrowRight => Action::Text(event::Text::MoveCursorRight),
        //     ),
        // );
        //

        let bindings = bi_hash_map_of!(
            KeyCode::ArrowDown => Action::Global(event::GlobalAction::Move(Direction::Down)),
            KeyCode::ArrowUp => Action::Global(event::GlobalAction::Move(Direction::Up)),
            KeyCode::ArrowLeft => Action::Global(event::GlobalAction::Move(Direction::Left)),
            KeyCode::ArrowRight => Action::Global(event::GlobalAction::Move(Direction::Right)),
        );

        Keybindings { bindings }
    }
}
