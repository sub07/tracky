use crate::{
    event::{Event, EventAware},
    keybindings::InputContext,
    EventSender,
};

pub mod loading;
pub mod slider;

pub enum Popup {
    Slider(slider::Popup),
}

impl Popup {
    pub fn handle_event(&mut self, event: Event, event_tx: EventSender) -> Option<Event> {
        match self {
            Popup::Slider(popup) => popup.handle_event(event, event_tx),
        }
    }

    pub fn input_context(&self) -> InputContext {
        match self {
            Popup::Slider(popup) => popup.input_context(),
        }
    }
}
