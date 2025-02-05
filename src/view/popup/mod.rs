use crate::{
    event::{Event, EventAware},
    keybindings::InputContext,
    EventSender,
};

pub mod input;
pub mod loading;

pub enum Popup {
    Input(input::Popup),
}

impl Popup {
    pub fn handle_event(&mut self, event: Event, event_tx: EventSender) -> Option<Event> {
        match self {
            Popup::Input(popup) => popup.handle_event(event, event_tx),
        }
    }

    pub fn input_context(&self) -> InputContext {
        match self {
            Popup::Input(popup) => popup.input_context(),
        }
    }
}
