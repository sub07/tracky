use ratatui::{layout::Rect, Frame};

use crate::{
    event::{Action, HandleAction},
    keybindings::InputContext,
    EventSender,
};

pub mod change_volume;
pub mod loading;

pub enum Popup {
    ChangeVolume(change_volume::Popup),
}

// TODO: Use macro to auto impl those methods
impl Popup {
    pub fn handle_event(&mut self, action: Action, event_tx: EventSender) {
        match self {
            Popup::ChangeVolume(popup) => {
                popup.handle_action(action, event_tx);
            }
        }
    }

    pub fn input_context(&self) -> InputContext {
        match self {
            Popup::ChangeVolume(popup) => popup.input_type(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        match self {
            Popup::ChangeVolume(popup) => popup.render(frame, area),
        }
    }
}
