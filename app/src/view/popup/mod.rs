use audio_device_selection::AudioDeviceSelectionPopup;

use crate::keybindings::Action;

pub mod audio_device_selection;

pub enum Popup {
    AudioDeviceSelection(AudioDeviceSelectionPopup),
}

impl Popup {
    pub fn handle_action(&mut self, action: Action, consumed: &mut bool) -> Option<Action> {
        match self {
            Popup::AudioDeviceSelection(popup) => popup.handle_action(action, consumed),
        }
    }
}
