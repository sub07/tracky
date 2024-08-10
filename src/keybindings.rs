use std::collections::HashMap;

use joy_collection_utils::hash_map_of;
use joy_impl_ignore::{debug::DebugImplIgnore, eq::PartialEqImplIgnore};
use joy_macro::DisplayFromDebug;
use ratatui::crossterm::event::KeyCode;

use crate::{
    audio,
    model::{
        pattern::{HexDigit, NoteName, OctaveValue},
        Direction,
    },
};

#[derive(PartialEq, Clone, Debug, DisplayFromDebug)]
pub enum Action {
    Note {
        note_name: NoteName,
        octave_modifier: i32,
    },
    Hex(HexDigit),
    Octave(OctaveValue),
    ClearField,
    Move(Direction, u32),
    InsertPattern,
    NextPattern,
    PreviousPattern,
    TogglePlay,
    NoteCut,
    ModifyDefaultOctave(i32),
    WriteLogsOnDisk,
    ClearLogsPanel,
    ToggleLogsPanel,
    Confirm,
    Cancel,
    ClosePopup,
    OpenDeviceSelectionPopup,
    SetPlayingDevice(DebugImplIgnore<PartialEqImplIgnore<audio::device::Device>>),
    ExitApp,
    Composite(Vec<Action>),
}

impl Action {
    const fn note(n: NoteName, o: i32) -> Action {
        Action::Note {
            note_name: n,
            octave_modifier: o,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum InputContext {
    Popup,
    Note,
    Octave,
    Hex,
    Global,
}

#[derive(Debug)]
pub struct KeyBindings {
    context_bindings: HashMap<InputContext, HashMap<KeyCode, Action>>,
}

// impl KeyBindings {
//     pub fn action(
//         &self,
//         modifiers: KeyModifiers,
//         key: KeyCode,
//         input_context: InputContext,
//     ) -> Option<Action> {
//         let keyboard_event = (modifiers, key);

//         if let Some(key_map) = self.context_bindings.get(&input_context) {
//             if let Some(action) = key_map.get(&keyboard_event) {
//                 return Some(*action);
//             } else if let Some(global_keybinds) =
//                 self.context_bindings.get(&InputContext::Global)
//             {
//                 if let Some(action) = global_keybinds.get(&keyboard_event) {
//                     return Some(*action);
//                 }
//             }
//         }
//         None
//     }
// }

impl KeyBindings {
    pub fn action(&self, key_code: KeyCode, input_context: InputContext) -> Option<Action> {
        fn get_action(
            bindings: &HashMap<InputContext, HashMap<KeyCode, Action>>,
            key_code: KeyCode,
            input_context: InputContext,
        ) -> Option<Action> {
            bindings
                .get(&input_context)
                .and_then(|bindings| bindings.get(&key_code))
                .cloned()
        }

        fn get_global_action(
            bindings: &HashMap<InputContext, HashMap<KeyCode, Action>>,
            key_code: KeyCode,
        ) -> Option<Action> {
            get_action(bindings, key_code, InputContext::Global)
        }

        fn get_or_global(
            bindings: &HashMap<InputContext, HashMap<KeyCode, Action>>,
            key_code: KeyCode,
            input_context: InputContext,
        ) -> Option<Action> {
            get_action(bindings, key_code, input_context)
                .or_else(|| get_global_action(bindings, key_code))
        }

        match input_context {
            InputContext::Global => get_global_action(&self.context_bindings, key_code),
            _ => get_or_global(&self.context_bindings, key_code, input_context),
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let context_bindings = hash_map_of!(
            InputContext::Note => hash_map_of!(
                KeyCode::Char('a') => Action::note(NoteName::C, 0),
                KeyCode::Char('é') => Action::note(NoteName::CSharp, 0),
                KeyCode::Char('z') => Action::note(NoteName::D, 0),
                KeyCode::Char('"') => Action::note(NoteName::DSharp, 0),
                KeyCode::Char('e') => Action::note(NoteName::E, 0),
                KeyCode::Char('r') => Action::note(NoteName::F, 0),
                KeyCode::Char('(') => Action::note(NoteName::FSharp, 0),
                KeyCode::Char('t') => Action::note(NoteName::G, 0),
                KeyCode::Char('-') => Action::note(NoteName::GSharp, 0),
                KeyCode::Char('y') => Action::note(NoteName::A, 0),
                KeyCode::Char('è') => Action::note(NoteName::ASharp, 0),
                KeyCode::Char('u') => Action::note(NoteName::B, 0),
                KeyCode::Char('&') => Action::NoteCut,
            ),
            InputContext::Octave => hash_map_of!(
                KeyCode::Char('à') => Action::Octave(OctaveValue::OCTAVE_0),
                KeyCode::Char('&') => Action::Octave(OctaveValue::OCTAVE_1),
                KeyCode::Char('é') => Action::Octave(OctaveValue::OCTAVE_2),
                KeyCode::Char('"') => Action::Octave(OctaveValue::OCTAVE_3),
                KeyCode::Char('\'') => Action::Octave(OctaveValue::OCTAVE_4),
                KeyCode::Char('(') => Action::Octave(OctaveValue::OCTAVE_5),
                KeyCode::Char('-') => Action::Octave(OctaveValue::OCTAVE_6),
                KeyCode::Char('è') => Action::Octave(OctaveValue::OCTAVE_7),
                KeyCode::Char('_') => Action::Octave(OctaveValue::OCTAVE_8),
                KeyCode::Char('ç') => Action::Octave(OctaveValue::OCTAVE_9),
            ),
            InputContext::Hex => hash_map_of!(
                KeyCode::Char('à') => Action::Hex(HexDigit::HEX_0),
                KeyCode::Char('&') => Action::Hex(HexDigit::HEX_1),
                KeyCode::Char('é') => Action::Hex(HexDigit::HEX_2),
                KeyCode::Char('"') => Action::Hex(HexDigit::HEX_3),
                KeyCode::Char('\'') => Action::Hex(HexDigit::HEX_4),
                KeyCode::Char('(') => Action::Hex(HexDigit::HEX_5),
                KeyCode::Char('-') => Action::Hex(HexDigit::HEX_6),
                KeyCode::Char('è') => Action::Hex(HexDigit::HEX_7),
                KeyCode::Char('_') => Action::Hex(HexDigit::HEX_8),
                KeyCode::Char('ç') => Action::Hex(HexDigit::HEX_9),
                KeyCode::Char('a') => Action::Hex(HexDigit::HEX_A),
                KeyCode::Char('b') => Action::Hex(HexDigit::HEX_B),
                KeyCode::Char('c') => Action::Hex(HexDigit::HEX_C),
                KeyCode::Char('d') => Action::Hex(HexDigit::HEX_D),
                KeyCode::Char('e') => Action::Hex(HexDigit::HEX_E),
                KeyCode::Char('f') => Action::Hex(HexDigit::HEX_F),
            ),
            InputContext::Popup => hash_map_of!(
                KeyCode::Down => Action::Move(Direction::Down, 1),
                KeyCode::Up => Action::Move(Direction::Up, 1),
                KeyCode::Left => Action::Move(Direction::Left, 1),
                KeyCode::Right => Action::Move(Direction::Right, 1),
                KeyCode::Tab => Action::Move(Direction::Right, 1),
                KeyCode::BackTab => Action::Move(Direction::Left, 1),
                KeyCode::Enter => Action::Confirm,
                KeyCode::Esc => Action::Cancel,
            ),
            InputContext::Global => hash_map_of!(
                KeyCode::Down => Action::Move(Direction::Down, 1),
                KeyCode::Up => Action::Move(Direction::Up, 1),
                KeyCode::Left => Action::Move(Direction::Left, 1),
                KeyCode::Right => Action::Move(Direction::Right, 1),
                KeyCode::Insert => Action::InsertPattern,
                KeyCode::Char('+') => Action::NextPattern,
                KeyCode::Char('-') => Action::PreviousPattern,
                KeyCode::Delete => Action::ClearField,
                KeyCode::Char(' ') => Action::TogglePlay,
                KeyCode::Char('*')=> Action::ModifyDefaultOctave(1),
                KeyCode::Char('/') => Action::ModifyDefaultOctave(-1),
                KeyCode::Esc => Action::ExitApp,
                KeyCode::F(9) => Action::WriteLogsOnDisk,
                KeyCode::F(10) => Action::ClearLogsPanel,
                KeyCode::F(12) => Action::ToggleLogsPanel,
                KeyCode::F(1) => Action::OpenDeviceSelectionPopup,
            ),
        );

        KeyBindings { context_bindings }
    }
}
