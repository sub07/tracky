use crate::{
    event::{Action, HandleAction},
    keybindings::InputContext,
    EventSender,
};

pub mod device_selection;
pub mod song_editor;

#[derive(Default, Debug)]
pub enum Screen {
    DeviceSelection(device_selection::State),
    #[default]
    SongEditor,
}

impl Screen {
    pub fn handle_event(&mut self, action: Action, event_tx: EventSender) {
        match self {
            Screen::DeviceSelection(state) => state.handle_action(action, event_tx),
            Screen::SongEditor => todo!(),
        }
    }

    pub fn input_context(&self) -> InputContext {
        match self {
            Screen::DeviceSelection(state) => state.input_context(),
            Screen::SongEditor => todo!(),
        }
    }
}
