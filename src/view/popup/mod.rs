use crate::{
    event::{Event, EventAware},
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
    pub fn handle_event(&mut self, event: Event, event_tx: EventSender) -> Option<Event> {
        match self {
            Popup::ChangeVolume(popup) => popup.handle_event(event, event_tx),
        }
    }

    pub fn input_context(&self) -> InputContext {
        match self {
            Popup::ChangeVolume(popup) => popup.input_context(),
        }
    }
}
