use std::collections::HashMap;

use joy_collection_utils::hash_map_of;
use winit::keyboard::KeyCode;

use crate::{
    event::{self, Event},
    model::{
        self,
        pattern::{HexDigit, NoteName, OctaveValue},
    },
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

type EventProducer = Box<dyn Fn() -> Event>;

pub struct KeyBindings {
    context_bindings: HashMap<InputContext, HashMap<KeyCode, EventProducer>>,
}

impl KeyBindings {
    pub fn action(&self, key: KeyCode, input_context: InputContext) -> Option<Event> {
        let get_action = |input_context| {
            self.context_bindings
                .get(&input_context)
                .and_then(|bindings| bindings.get(&key))
                .map(|event_producer| event_producer())
        };

        match input_context {
            InputContext::Global => get_action(InputContext::Global),
            _ => get_action(input_context).or_else(|| get_action(InputContext::Global)),
        }
    }
}

fn song_note_event(n: NoteName, o: i32) -> Event {
    Event::State(model::Event::SetNoteField {
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
                KeyCode::KeyQ => b!(song_note_event(NoteName::C, 0)),
                KeyCode::Digit2 => b!(song_note_event(NoteName::CSharp, 0)),
                KeyCode::KeyW => b!(song_note_event(NoteName::D, 0)),
                KeyCode::Digit3 => b!(song_note_event(NoteName::DSharp, 0)),
                KeyCode::KeyE => b!(song_note_event(NoteName::E, 0)),
                KeyCode::KeyR => b!(song_note_event(NoteName::F, 0)),
                KeyCode::Digit5 => b!(song_note_event(NoteName::FSharp, 0)),
                KeyCode::KeyT => b!(song_note_event(NoteName::G, 0)),
                KeyCode::Digit6 => b!(song_note_event(NoteName::GSharp, 0)),
                KeyCode::KeyY => b!(song_note_event(NoteName::A, 0)),
                KeyCode::Digit7 => b!(song_note_event(NoteName::ASharp, 0)),
                KeyCode::KeyU => b!(song_note_event(NoteName::B, 0)),
                KeyCode::Digit1 => b!(Event::State(model::Event::SetNoteFieldToCut)),
            ),
            InputContext::Octave => hash_map_of!(
                KeyCode::Digit0 => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_0))),
                KeyCode::Digit1 => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_1))),
                KeyCode::Digit2 => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_2))),
                KeyCode::Digit3 => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_3))),
                KeyCode::Digit4 => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_4))),
                KeyCode::Digit5 => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_5))),
                KeyCode::Digit6 => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_6))),
                KeyCode::Digit7 => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_7))),
                KeyCode::Digit8 => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_8))),
                KeyCode::Digit9 => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_9))),
            ),
            InputContext::Hex => hash_map_of!(
                KeyCode::Digit0 => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_0))),
                KeyCode::Digit1 => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_1))),
                KeyCode::Digit2 => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_2))),
                KeyCode::Digit3 => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_3))),
                KeyCode::Digit4 => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_4))),
                KeyCode::Digit5 => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_5))),
                KeyCode::Digit6 => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_6))),
                KeyCode::Digit7 => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_7))),
                KeyCode::Digit8 => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_8))),
                KeyCode::Digit9 => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_9))),
            ),
            InputContext::Popup => hash_map_of!(
                KeyCode::ArrowDown => b!(Event::Action(event::Action::Move(Direction::Down))),
                KeyCode::ArrowUp => b!(Event::Action(event::Action::Move(Direction::Up))),
                KeyCode::ArrowLeft => b!(Event::Action(event::Action::Move(Direction::Left))),
                KeyCode::ArrowRight => b!(Event::Action(event::Action::Move(Direction::Right))),
                KeyCode::Tab => b!(Event::Action(event::Action::Forward)),
                KeyCode::Enter => b!(Event::Action(event::Action::Confirm)),
                KeyCode::Escape => b!(Event::Action(event::Action::Cancel)),
            ),
            InputContext::Global => hash_map_of!(
                KeyCode::ArrowDown => b!(Event::Action(event::Action::Move(Direction::Down))),
                KeyCode::ArrowUp => b!(Event::Action(event::Action::Move(Direction::Up))),
                KeyCode::ArrowLeft => b!(Event::Action(event::Action::Move(Direction::Left))),
                KeyCode::ArrowRight => b!(Event::Action(event::Action::Move(Direction::Right))),
                KeyCode::Insert => b!(Event::State(model::Event::NewPattern)),
                KeyCode::NumpadAdd => b!(Event::State(model::Event::NextPattern)),
                KeyCode::NumpadSubtract => b!(Event::State(model::Event::PreviousPattern)),
                KeyCode::Delete => b!(Event::State(model::Event::ClearField)),
                KeyCode::Space => b!(Event::Action(event::Action::TogglePlay)),
                KeyCode::NumpadMultiply => b!(Event::State(model::Event::MutateGlobalOctave { increment: 1 })),
                KeyCode::NumpadDivide => b!(Event::State(model::Event::MutateGlobalOctave { increment: -1 })),
                KeyCode::Escape => b!(Event::Action(event::Action::Cancel)),
                KeyCode::Enter => b!(Event::Action(event::Action::Confirm)),
                KeyCode::F1 => b!(Event::Action(event::Action::RequestOpenDeviceSelectionPopup)),
                KeyCode::F11 => b!(Event::Action(event::Action::ToggleFullscreen)),
            ),
            InputContext::Text => hash_map_of!(
                KeyCode::Backspace => b!(Event::Text(event::Text::RemoveCharAtCursor)),
                KeyCode::ArrowLeft => b!(Event::Text(event::Text::MoveCursorLeft)),
                KeyCode::ArrowRight => b!(Event::Text(event::Text::MoveCursorRight)),
            ),
        );

        KeyBindings { context_bindings }
    }
}
