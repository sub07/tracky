use std::sync::mpsc::Sender;

use crate::event::Event;

pub mod audio_device_selection;

pub enum Popup {
    AudioDeviceSelection(audio_device_selection::Popup),
}

impl Popup {
    pub fn handle_event(&mut self, event: Event, event_tx: Sender<Event>) -> Option<Event> {
        match self {
            Popup::AudioDeviceSelection(popup) => {
                if let Some(popup_event) = popup.map_event(&event) {
                    popup.handle_event(popup_event, event_tx);
                    None
                } else {
                    Some(event)
                }
            }
        }
    }
}
