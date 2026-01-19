use crate::{
    app::Tracky,
    event::{Action, HandleAction},
    keybindings::InputContext,
};

pub mod change_volume;
pub mod loading;

pub enum Popup {
    ChangeVolume(change_volume::Popup),
}

impl Tracky {
    pub fn open_popup(&mut self, popup: Popup) {
        self.current_popup = Some(popup);
    }

    pub fn close_popup(&mut self) {
        self.current_popup = None;
    }

    pub fn popup_input_context(&self) -> Option<InputContext> {
        self.current_popup.as_ref().map(|popup| match popup {
            Popup::ChangeVolume(popup) => popup.input_context(),
        })
    }

    pub fn handle_action_on_popup(&mut self, action: Action) -> Option<Action> {
        if let Some(popup) = &mut self.current_popup {
            match popup {
                Popup::ChangeVolume(popup) => {
                    popup.handle_action(action, self.event_sender.clone());
                }
            }
            None
        } else {
            Some(action)
        }
    }
}
