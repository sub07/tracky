use crate::audio::{Device, Hosts};

#[derive(Debug)]
pub enum Event {
    Key(ratatui::crossterm::event::KeyEvent),
    Action(crate::keybindings::Action),
    Panic(anyhow::Error),
    AsyncAction(AsyncAction),
    Resize { width: u16, height: u16 },
    Composite(Vec<Event>),
    StartLoading,
    LoadingDone(AsyncAction),
    ClosePopup,
    SetPlayingDevice(Device),
}

#[derive(Debug)]
pub enum AsyncAction {
    OpenDeviceSelectionPopup(Hosts),
}
