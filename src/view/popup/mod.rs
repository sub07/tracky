use crate::{event::Event, keybindings::Action};

pub mod audio_device_selection;

pub enum Popup {
    AudioDeviceSelection(audio_device_selection::Popup),
}

impl Popup {
    pub fn handle_event(&mut self, event: Event) -> Option<Event> {
        match self {
            Popup::AudioDeviceSelection(popup) => {
                if let Some(event) = popup.map_event(&event) {
                    popup.handle_event(event)
                } else {
                    Some(event)
                }
            }
        }
    }
}
