use std::collections::HashMap;

use joy_collection_utils::hash_map_of;
use ratatui::crossterm::event::KeyCode;

use crate::{
    event::{self, Event},
    model::{
        self,
        pattern::{HexDigit, NoteName, OctaveValue},
    },
    utils::Direction,
};

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum InputContext {
    Popup,
    Note,
    Octave,
    Hex,
    Global,
}

type EventProducer = Box<dyn Fn() -> Event>;

pub struct KeyBindings {
    context_bindings: HashMap<InputContext, HashMap<KeyCode, EventProducer>>,
}

impl KeyBindings {
    pub fn action(&self, key_code: KeyCode, input_context: InputContext) -> Option<Event> {
        let get_action = |input_context| {
            self.context_bindings
                .get(&input_context)
                .and_then(|bindings| bindings.get(&key_code))
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
                KeyCode::Char('a') => b!(song_note_event(NoteName::C, 0)),
                KeyCode::Char('é') => b!(song_note_event(NoteName::CSharp, 0)),
                KeyCode::Char('z') => b!(song_note_event(NoteName::D, 0)),
                KeyCode::Char('"') => b!(song_note_event(NoteName::DSharp, 0)),
                KeyCode::Char('e') => b!(song_note_event(NoteName::E, 0)),
                KeyCode::Char('r') => b!(song_note_event(NoteName::F, 0)),
                KeyCode::Char('(') => b!(song_note_event(NoteName::FSharp, 0)),
                KeyCode::Char('t') => b!(song_note_event(NoteName::G, 0)),
                KeyCode::Char('-') => b!(song_note_event(NoteName::GSharp, 0)),
                KeyCode::Char('y') => b!(song_note_event(NoteName::A, 0)),
                KeyCode::Char('è') => b!(song_note_event(NoteName::ASharp, 0)),
                KeyCode::Char('u') => b!(song_note_event(NoteName::B, 0)),
                KeyCode::Char('&') => b!(Event::State(model::Event::SetNoteFieldToCut)),
            ),
            InputContext::Octave => hash_map_of!(
                KeyCode::Char('à') => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_0))),
                KeyCode::Char('&') => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_1))),
                KeyCode::Char('é') => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_2))),
                KeyCode::Char('"') => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_3))),
                KeyCode::Char('\'') => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_4))),
                KeyCode::Char('(') => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_5))),
                KeyCode::Char('-') => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_6))),
                KeyCode::Char('è') => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_7))),
                KeyCode::Char('_') => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_8))),
                KeyCode::Char('ç') => b!(Event::State(model::Event::SetOctaveField(OctaveValue::OCTAVE_9))),
            ),
            InputContext::Hex => hash_map_of!(
                KeyCode::Char('à') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_0))),
                KeyCode::Char('&') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_1))),
                KeyCode::Char('é') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_2))),
                KeyCode::Char('"') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_3))),
                KeyCode::Char('\'') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_4))),
                KeyCode::Char('(') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_5))),
                KeyCode::Char('-') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_6))),
                KeyCode::Char('è') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_7))),
                KeyCode::Char('_') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_8))),
                KeyCode::Char('ç') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_9))),
                KeyCode::Char('a') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_A))),
                KeyCode::Char('b') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_B))),
                KeyCode::Char('c') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_C))),
                KeyCode::Char('d') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_D))),
                KeyCode::Char('e') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_E))),
                KeyCode::Char('f') => b!(Event::State(model::Event::SetHexField(HexDigit::HEX_F))),
            ),
            InputContext::Popup => hash_map_of!(
                KeyCode::Down => b!(Event::Action(event::Action::Move(Direction::Down))),
                KeyCode::Up => b!(Event::Action(event::Action::Move(Direction::Up))),
                KeyCode::Left => b!(Event::Action(event::Action::Move(Direction::Left))),
                KeyCode::Right => b!(Event::Action(event::Action::Move(Direction::Right))),
                KeyCode::Tab => b!(Event::Action(event::Action::Forward)),
                KeyCode::BackTab => b!(Event::Action(event::Action::Backward)),
                KeyCode::Enter => b!(Event::Action(event::Action::Confirm)),
                KeyCode::Esc => b!(Event::Action(event::Action::Cancel)),
            ),
            InputContext::Global => hash_map_of!(
                KeyCode::Down => b!(Event::Action(event::Action::Move(Direction::Down))),
                KeyCode::Up => b!(Event::Action(event::Action::Move(Direction::Up))),
                KeyCode::Left => b!(Event::Action(event::Action::Move(Direction::Left))),
                KeyCode::Right => b!(Event::Action(event::Action::Move(Direction::Right))),
                KeyCode::Insert => b!(Event::State(model::Event::NewPattern)),
                KeyCode::Char('+') => b!(Event::State(model::Event::NextPattern)),
                KeyCode::Char('-') => b!(Event::State(model::Event::PreviousPattern)),
                KeyCode::Delete => b!(Event::State(model::Event::ClearField)),
                KeyCode::Char(' ') => b!(Event::Action(event::Action::TogglePlay)),
                KeyCode::Char('*')=> b!(Event::State(model::Event::MutateGlobalOctave { increment: 1 })),
                KeyCode::Char('/') => b!(Event::State(model::Event::MutateGlobalOctave { increment: -1 })),
                KeyCode::Esc => b!(Event::ExitApp),
                KeyCode::F(9) => b!(Event::Action(event::Action::WriteLogsOnDisk)),
                KeyCode::F(10) => b!(Event::Action(event::Action::ClearLogsPanel)),
                KeyCode::F(12) => b!(Event::Action(event::Action::ToggleLogsPanel)),
                KeyCode::F(1) => b!(Event::Action(event::Action::RequestOpenDeviceSelectionPopup)),
            ),
        );

        KeyBindings { context_bindings }
    }
}
