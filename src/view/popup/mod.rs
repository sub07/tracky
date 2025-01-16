use std::sync::mpsc::Sender;

use crate::{event::Event, keybindings::InputContext};

pub mod audio_device_selection;
pub mod input;
pub mod loading;

trait HandleEvent<PopupEvent> {
    fn map_event(&self, event: &Event) -> Option<PopupEvent>;
    fn update(&mut self, event: PopupEvent, event_tx: Sender<Event>);
    fn input_context(&self) -> InputContext;
    fn handle_event(&mut self, event: Event, event_tx: Sender<Event>) -> Option<Event> {
        if let Some(popup_event) = self.map_event(&event) {
            self.update(popup_event, event_tx);
            None
        } else {
            Some(event)
        }
    }
}

pub enum Popup {
    AudioDeviceSelection(audio_device_selection::Popup),
    Input(input::Popup),
}

impl Popup {
    pub fn handle_event(&mut self, event: Event, event_tx: Sender<Event>) -> Option<Event> {
        match self {
            Popup::AudioDeviceSelection(popup) => popup.handle_event(event, event_tx),
            Popup::Input(popup) => popup.handle_event(event, event_tx),
        }
    }

    pub fn input_context(&self) -> InputContext {
        match self {
            Popup::AudioDeviceSelection(popup) => popup.input_context(),
            Popup::Input(popup) => popup.input_context(),
        }
    }
}
