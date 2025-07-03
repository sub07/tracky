use std::{collections::HashMap, hash::Hash};

use joy_collection_utils::hash_map_of;
use winit::keyboard::{KeyCode, ModifiersState};

use crate::{
    event::{self, Action},
    model::pattern::{HexDigit, NoteName, OctaveValue},
    utils::Direction,
};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum InputContext {
    Popup,
    Note,
    Octave,
    Hex,
    Global,
    Text,
}

#[derive(PartialEq, Eq, Hash)]
struct Keybinding(ModifiersState, KeyCode);

impl From<KeyCode> for Keybinding {
    fn from(key: KeyCode) -> Self {
        Keybinding(ModifiersState::empty(), key)
    }
}

impl From<(ModifiersState, KeyCode)> for Keybinding {
    fn from((modifiers, key): (ModifiersState, KeyCode)) -> Self {
        Keybinding(modifiers, key)
    }
}

pub struct Keybindings {
    context_bindings: HashMap<InputContext, HashMap<Keybinding, Action>>,
}

impl Keybindings {
    pub fn action(
        &self,
        modifiers: ModifiersState,
        key: KeyCode,
        input_context: InputContext,
    ) -> Option<Action> {
        let get_action = |input_context| {
            self.context_bindings
                .get(&input_context)
                .and_then(|bindings| bindings.get(&(modifiers, key).into()))
                .cloned()
        };

        match input_context {
            InputContext::Global => get_action(InputContext::Global),
            _ => get_action(input_context).or_else(|| get_action(InputContext::Global)),
        }
    }
}

fn song_note_event(n: NoteName, o: i32) -> Action {
    Action::SetNoteField {
        note: n,
        octave_modifier: o,
    }
}

impl Default for Keybindings {
    fn default() -> Self {
        let context_bindings = hash_map_of!(
            InputContext::Note => hash_map_of!(
                KeyCode::KeyQ => song_note_event(NoteName::C, 0),
                KeyCode::Digit2 => song_note_event(NoteName::CSharp, 0),
                KeyCode::KeyW => song_note_event(NoteName::D, 0),
                KeyCode::Digit3 => song_note_event(NoteName::DSharp, 0),
                KeyCode::KeyE => song_note_event(NoteName::E, 0),
                KeyCode::KeyR => song_note_event(NoteName::F, 0),
                KeyCode::Digit5 => song_note_event(NoteName::FSharp, 0),
                KeyCode::KeyT => song_note_event(NoteName::G, 0),
                KeyCode::Digit6 => song_note_event(NoteName::GSharp, 0),
                KeyCode::KeyY => song_note_event(NoteName::A, 0),
                KeyCode::Digit7 => song_note_event(NoteName::ASharp, 0),
                KeyCode::KeyU => song_note_event(NoteName::B, 0),
                KeyCode::Digit1 => Action::SetNoteCut,
            ),
            InputContext::Octave => hash_map_of!(
                KeyCode::Digit0 => Action::SetOctaveField(OctaveValue::OCTAVE_0),
                KeyCode::Digit1 => Action::SetOctaveField(OctaveValue::OCTAVE_1),
                KeyCode::Digit2 => Action::SetOctaveField(OctaveValue::OCTAVE_2),
                KeyCode::Digit3 => Action::SetOctaveField(OctaveValue::OCTAVE_3),
                KeyCode::Digit4 => Action::SetOctaveField(OctaveValue::OCTAVE_4),
                KeyCode::Digit5 => Action::SetOctaveField(OctaveValue::OCTAVE_5),
                KeyCode::Digit6 => Action::SetOctaveField(OctaveValue::OCTAVE_6),
                KeyCode::Digit7 => Action::SetOctaveField(OctaveValue::OCTAVE_7),
                KeyCode::Digit8 => Action::SetOctaveField(OctaveValue::OCTAVE_8),
                KeyCode::Digit9 => Action::SetOctaveField(OctaveValue::OCTAVE_9),
            ),
            InputContext::Hex => hash_map_of!(
                KeyCode::Digit0 => Action::SetHexField(HexDigit::HEX_0),
                KeyCode::Digit1 => Action::SetHexField(HexDigit::HEX_1),
                KeyCode::Digit2 => Action::SetHexField(HexDigit::HEX_2),
                KeyCode::Digit3 => Action::SetHexField(HexDigit::HEX_3),
                KeyCode::Digit4 => Action::SetHexField(HexDigit::HEX_4),
                KeyCode::Digit5 => Action::SetHexField(HexDigit::HEX_5),
                KeyCode::Digit6 => Action::SetHexField(HexDigit::HEX_6),
                KeyCode::Digit7 => Action::SetHexField(HexDigit::HEX_7),
                KeyCode::Digit8 => Action::SetHexField(HexDigit::HEX_8),
                KeyCode::Digit9 => Action::SetHexField(HexDigit::HEX_9),
            ),
            InputContext::Popup => hash_map_of!(
                KeyCode::ArrowDown => Action::Move(Direction::Down),
                KeyCode::ArrowUp => Action::Move(Direction::Up),
                KeyCode::ArrowLeft => Action::Move(Direction::Left),
                KeyCode::ArrowRight => Action::Move(Direction::Right),
                KeyCode::Tab => Action::Forward,
                KeyCode::Enter => Action::Confirm,
                KeyCode::Escape => Action::Cancel,
            ),
            InputContext::Global => hash_map_of!(
                KeyCode::ArrowDown => Action::Move(Direction::Down),
                KeyCode::ArrowUp => Action::Move(Direction::Up),
                KeyCode::ArrowLeft => Action::Move(Direction::Left),
                KeyCode::ArrowRight => Action::Move(Direction::Right),
                KeyCode::Insert => Action::CreateNewPattern,
                KeyCode::NumpadAdd => Action::GoToNextPattern,
                KeyCode::NumpadSubtract => Action::GoToPreviousPattern,
                KeyCode::Delete => Action::ClearField,
                KeyCode::Space => Action::TogglePlay,
                KeyCode::NumpadMultiply => Action::ChangeGlobalOctave { increment: 1 },
                KeyCode::NumpadDivide => Action::ChangeGlobalOctave { increment: -1 },
                KeyCode::Escape => Action::Cancel,
                KeyCode::Enter => Action::Confirm,
                KeyCode::F1 => Action::RequestChangeScreenToDeviceSelection,
                KeyCode::F2 => Action::RequestChangeScreenToSongEditor,
                KeyCode::F11 => Action::ToggleFullscreen,
                KeyCode::F8 => Action::KillNotes,
                (ModifiersState::ALT, KeyCode::KeyV) => Action::ShowGlobalVolumePopup,
                KeyCode::PageDown => Action::ChangeSelectedInstrument { increment: 1 },
                KeyCode::PageUp => Action::ChangeSelectedInstrument { increment: -1 },
            ),
            InputContext::Text => hash_map_of!(
                KeyCode::Backspace => Action::Text(event::Text::RemoveCharAtCursor),
                KeyCode::ArrowLeft => Action::Text(event::Text::MoveCursorLeft),
                KeyCode::ArrowRight => Action::Text(event::Text::MoveCursorRight),
            ),
        );

        Keybindings { context_bindings }
    }
}
