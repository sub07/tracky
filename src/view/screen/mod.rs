use ratatui::{layout::Rect, Frame};

use crate::{
    event::{Action, GlobalAction, HandleAction},
    keybindings::{InputContext, InputType},
    tracky::Tracky,
    EventSender,
};

pub mod device_selection;
pub mod song_editor;

#[derive(Debug)]
pub enum Screen {
    DeviceSelection(device_selection::State),
    SongEditor(song_editor::State),
}

impl Default for Screen {
    fn default() -> Self {
        Screen::SongEditor(song_editor::State::default())
    }
}

pub enum ScreenAction<A> {
    Global(GlobalAction),
    Screen(A),
}

impl From<GlobalAction> for ScreenAction<GlobalAction> {
    fn from(value: GlobalAction) -> Self {
        ScreenAction::Global(value)
    }
}

impl Screen {
    pub fn update<'a>(&'a mut self, app: &'a Tracky, action: Action, event_tx: EventSender) {
        match self {
            Screen::DeviceSelection(state) => state.handle_action(action, event_tx),
            Screen::SongEditor(state) => state.render(action, event_tx),
        }
    }

    pub fn input_type(&self) -> InputType {
        match self {
            Screen::DeviceSelection(state) => state.input_type(),
            Screen::SongEditor(state) => state.input_type(),
        }
    }

    pub fn render(&mut self, app: &Tracky, frame: &mut Frame, area: Rect) {
        match self {
            Screen::DeviceSelection(state) => state.render(frame, area),
            Screen::SongEditor(state) => state.render(frame, area),
        }
    }
}
