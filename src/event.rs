use winit::event::KeyEvent;

use crate::{
    audio::{Device, Hosts},
    model::{self},
    utils::Direction,
    view::popup::input::InputId,
};

#[derive(Debug)]
pub enum Event {
    KeyPressed(KeyEvent),
    Text(Text),
    State(model::Event),
    AudioCallback(model::Event),
    Panic(anyhow::Error),
    Action(Action),
    AsyncAction(AsyncAction),
    Resize { width: u16, height: u16 },
    Composite(Vec<Event>),
    StartLoading,
    LoadingDone(AsyncAction),
    ClosePopup,
    SetPlayingDevice(Device),
    StartAudioPlayer,
    StopAudioPlayer(Option<anyhow::Error>),
    RequestRedraw,
    TextSubmitted(InputId, String),
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
    ToggleFullscreen,
    Confirm,
    Cancel,
    RequestOpenDeviceSelectionPopup,
}

#[derive(Debug, Clone)]
pub enum Text {
    WriteDataAtCursor(char),
    RemoveCharAtCursor,
    MoveCursorLeft,
    MoveCursorRight,
}
