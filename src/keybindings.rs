use std::{collections::HashMap, hash::Hash};

use joy_collection_utils::hash_map_of;
use winit::keyboard::{KeyCode, ModifiersState};

use crate::{
    event::{self, Event},
    model::{
        self,
        pattern::{HexDigit, NoteName, OctaveValue},
    },
    utils::Direction,
    view::screen::Screen,
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

type EventProducer = Box<dyn Fn() -> Event>;

pub struct KeyBindings {
    context_bindings: HashMap<InputContext, HashMap<(ModifiersState, KeyCode), EventProducer>>,
}

impl KeyBindings {
    pub fn action(
        &self,
        modifiers: ModifiersState,
        key: KeyCode,
        input_context: InputContext,
    ) -> Option<Event> {
        let get_action = |input_context| {
            self.context_bindings
                .get(&input_context)
                .and_then(|bindings| bindings.get(&(modifiers, key)))
                .map(|event_producer| event_producer())
        };

        match input_context {
            InputContext::Global => get_action(InputContext::Global),
            _ => get_action(input_context).or_else(|| get_action(InputContext::Global)),
        }
    }
}

fn song_note_event(n: NoteName, o: i32) -> Event {
    Event::State(model::Command::SetNoteField {
        note: n,
        octave_modifier: o,
    })
}

macro_rules! b {
    ($event:expr) => {
        Box::new(|| $event) as Box<dyn Fn() -> Event>
    };
}

impl Default for KeyBindings {
    fn default() -> Self {
        let context_bindings = hash_map_of!(
            InputContext::Note => hash_map_of!(
                (ModifiersState::empty(), KeyCode::KeyQ) => b!(song_note_event(NoteName::C, 0)),
                (ModifiersState::empty(), KeyCode::Digit2) => b!(song_note_event(NoteName::CSharp, 0)),
                (ModifiersState::empty(), KeyCode::KeyW) => b!(song_note_event(NoteName::D, 0)),
                (ModifiersState::empty(), KeyCode::Digit3) => b!(song_note_event(NoteName::DSharp, 0)),
                (ModifiersState::empty(), KeyCode::KeyE) => b!(song_note_event(NoteName::E, 0)),
                (ModifiersState::empty(), KeyCode::KeyR) => b!(song_note_event(NoteName::F, 0)),
                (ModifiersState::empty(), KeyCode::Digit5) => b!(song_note_event(NoteName::FSharp, 0)),
                (ModifiersState::empty(), KeyCode::KeyT) => b!(song_note_event(NoteName::G, 0)),
                (ModifiersState::empty(), KeyCode::Digit6) => b!(song_note_event(NoteName::GSharp, 0)),
                (ModifiersState::empty(), KeyCode::KeyY) => b!(song_note_event(NoteName::A, 0)),
                (ModifiersState::empty(), KeyCode::Digit7) => b!(song_note_event(NoteName::ASharp, 0)),
                (ModifiersState::empty(), KeyCode::KeyU) => b!(song_note_event(NoteName::B, 0)),
                (ModifiersState::empty(), KeyCode::Digit1) => b!(Event::State(model::Command::SetNoteFieldToCut)),
            ),
            InputContext::Octave => hash_map_of!(
                (ModifiersState::empty(), KeyCode::Digit0) => b!(Event::State(model::Command::SetOctaveField(OctaveValue::OCTAVE_0))),
                (ModifiersState::empty(), KeyCode::Digit1) => b!(Event::State(model::Command::SetOctaveField(OctaveValue::OCTAVE_1))),
                (ModifiersState::empty(), KeyCode::Digit2) => b!(Event::State(model::Command::SetOctaveField(OctaveValue::OCTAVE_2))),
                (ModifiersState::empty(), KeyCode::Digit3) => b!(Event::State(model::Command::SetOctaveField(OctaveValue::OCTAVE_3))),
                (ModifiersState::empty(), KeyCode::Digit4) => b!(Event::State(model::Command::SetOctaveField(OctaveValue::OCTAVE_4))),
                (ModifiersState::empty(), KeyCode::Digit5) => b!(Event::State(model::Command::SetOctaveField(OctaveValue::OCTAVE_5))),
                (ModifiersState::empty(), KeyCode::Digit6) => b!(Event::State(model::Command::SetOctaveField(OctaveValue::OCTAVE_6))),
                (ModifiersState::empty(), KeyCode::Digit7) => b!(Event::State(model::Command::SetOctaveField(OctaveValue::OCTAVE_7))),
                (ModifiersState::empty(), KeyCode::Digit8) => b!(Event::State(model::Command::SetOctaveField(OctaveValue::OCTAVE_8))),
                (ModifiersState::empty(), KeyCode::Digit9) => b!(Event::State(model::Command::SetOctaveField(OctaveValue::OCTAVE_9))),
            ),
            InputContext::Hex => hash_map_of!(
                (ModifiersState::empty(), KeyCode::Digit0) => b!(Event::State(model::Command::SetHexField(HexDigit::HEX_0))),
                (ModifiersState::empty(), KeyCode::Digit1) => b!(Event::State(model::Command::SetHexField(HexDigit::HEX_1))),
                (ModifiersState::empty(), KeyCode::Digit2) => b!(Event::State(model::Command::SetHexField(HexDigit::HEX_2))),
                (ModifiersState::empty(), KeyCode::Digit3) => b!(Event::State(model::Command::SetHexField(HexDigit::HEX_3))),
                (ModifiersState::empty(), KeyCode::Digit4) => b!(Event::State(model::Command::SetHexField(HexDigit::HEX_4))),
                (ModifiersState::empty(), KeyCode::Digit5) => b!(Event::State(model::Command::SetHexField(HexDigit::HEX_5))),
                (ModifiersState::empty(), KeyCode::Digit6) => b!(Event::State(model::Command::SetHexField(HexDigit::HEX_6))),
                (ModifiersState::empty(), KeyCode::Digit7) => b!(Event::State(model::Command::SetHexField(HexDigit::HEX_7))),
                (ModifiersState::empty(), KeyCode::Digit8) => b!(Event::State(model::Command::SetHexField(HexDigit::HEX_8))),
                (ModifiersState::empty(), KeyCode::Digit9) => b!(Event::State(model::Command::SetHexField(HexDigit::HEX_9))),
            ),
            InputContext::Popup => hash_map_of!(
                (ModifiersState::empty(), KeyCode::ArrowDown) => b!(Event::Action(event::Action::Move(Direction::Down))),
                (ModifiersState::empty(), KeyCode::ArrowUp) => b!(Event::Action(event::Action::Move(Direction::Up))),
                (ModifiersState::empty(), KeyCode::ArrowLeft) => b!(Event::Action(event::Action::Move(Direction::Left))),
                (ModifiersState::empty(), KeyCode::ArrowRight) => b!(Event::Action(event::Action::Move(Direction::Right))),
                (ModifiersState::empty(), KeyCode::Tab) => b!(Event::Action(event::Action::Forward)),
                (ModifiersState::empty(), KeyCode::Enter) => b!(Event::Action(event::Action::Confirm)),
                (ModifiersState::empty(), KeyCode::Escape) => b!(Event::Action(event::Action::Cancel)),
            ),
            InputContext::Global => hash_map_of!(
                (ModifiersState::empty(), KeyCode::ArrowDown) => b!(Event::Action(event::Action::Move(Direction::Down))),
                (ModifiersState::empty(), KeyCode::ArrowUp) => b!(Event::Action(event::Action::Move(Direction::Up))),
                (ModifiersState::empty(), KeyCode::ArrowLeft) => b!(Event::Action(event::Action::Move(Direction::Left))),
                (ModifiersState::empty(), KeyCode::ArrowRight) => b!(Event::Action(event::Action::Move(Direction::Right))),
                (ModifiersState::empty(), KeyCode::Insert) => b!(Event::State(model::Command::CreateNewPattern)),
                (ModifiersState::empty(), KeyCode::NumpadAdd) => b!(Event::State(model::Command::GoToNextPattern)),
                (ModifiersState::empty(), KeyCode::NumpadSubtract) => b!(Event::State(model::Command::GoToPreviousPattern)),
                (ModifiersState::empty(), KeyCode::Delete) => b!(Event::State(model::Command::ClearField)),
                (ModifiersState::empty(), KeyCode::Space) => b!(Event::Action(event::Action::TogglePlay)),
                (ModifiersState::empty(), KeyCode::NumpadMultiply) => b!(Event::State(model::Command::ChangeGlobalOctave { increment: 1 })),
                (ModifiersState::empty(), KeyCode::NumpadDivide) => b!(Event::State(model::Command::ChangeGlobalOctave { increment: -1 })),
                (ModifiersState::empty(), KeyCode::Escape) => b!(Event::Action(event::Action::Cancel)),
                (ModifiersState::empty(), KeyCode::Enter) => b!(Event::Action(event::Action::Confirm)),
                (ModifiersState::empty(), KeyCode::F1) => b!(Event::Action(event::Action::RequestChangeScreenToDeviceSelection)),
                (ModifiersState::empty(), KeyCode::F2) => b!(Event::ChangeScreen(Screen::SongEditor)),
                (ModifiersState::empty(), KeyCode::F11) => b!(Event::Action(event::Action::ToggleFullscreen)),
                (ModifiersState::empty(), KeyCode::F8) => b!(Event::Action(event::Action::KillNotes)),
                (ModifiersState::empty(), KeyCode::PageDown) => b!(Event::Action(event::Action::ChangeSelectedInstrument { increment: 1 })),
                (ModifiersState::empty(), KeyCode::PageUp) => b!(Event::Action(event::Action::ChangeSelectedInstrument { increment: -1 })),
            ),
            InputContext::Text => hash_map_of!(
                (ModifiersState::empty(), KeyCode::Backspace) => b!(Event::Text(event::Text::RemoveCharAtCursor)),
                (ModifiersState::empty(), KeyCode::ArrowLeft) => b!(Event::Text(event::Text::MoveCursorLeft)),
                (ModifiersState::empty(), KeyCode::ArrowRight) => b!(Event::Text(event::Text::MoveCursorRight)),
            ),
        );

        KeyBindings { context_bindings }
    }
}
