use crate::{
    audio::{Device, Hosts},
    model::{song, Direction},
};

#[derive(Debug)]
pub enum Event {
    Key(ratatui::crossterm::event::KeyEvent),
    Song(song::Event),
    Panic(anyhow::Error),
    Action(Action),
    AsyncAction(AsyncAction),
    Resize { width: u16, height: u16 },
    Composite(Vec<Event>),
    StartLoading,
    LoadingDone(AsyncAction),
    ClosePopup,
    SetPlayingDevice(Device),
    ExitApp,
}

#[derive(Debug)]
pub enum AsyncAction {
    OpenDeviceSelectionPopup(Hosts),
}

#[derive(Debug)]
pub enum Action {
    Move(Direction),
    Forward,
    Backward,
    TogglePlay,
    WriteLogsOnDisk,
    ClearLogsPanel,
    ToggleLogsPanel,
    Confirm,
    Cancel,
    RequestOpenDeviceSelectionPopup,
}
