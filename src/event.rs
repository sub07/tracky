use winit::{event::KeyEvent, keyboard::ModifiersState};

use crate::{
    audio::device::{ConfiguredDevice, Devices},
    keybindings::{InputContext, InputType},
    model::{
        self,
        pattern::{HexDigit, NoteFieldValue, NoteName, OctaveValue},
    },
    tracky::Tracky,
    utils::Direction,
    view::screen::{self, Screen},
    EventSender,
};

#[derive(Debug)]
pub enum Event {
    KeyPressed(ModifiersState, KeyEvent),
    State(model::Command),
    AudioCallback(model::Command),
    Panic(anyhow::Error),
    Action(Action),
    AsyncAction(AsyncAction),
    Resize { width: u16, height: u16 },
    Composite(Vec<Event>),
    StartLoading,
    LoadingDone(AsyncAction),
    ClosePopup,
    SetPlayingDevice(ConfiguredDevice),
    StartAudioPlayer,
    StopAudioPlayer(Option<anyhow::Error>),
    RequestRedraw,
    Text(Text),
    ChangeScreen(Screen),
    ToggleFullscreen,
    ExitApp,
}

#[derive(Debug)]
pub enum AsyncAction {
    GetDevices(Devices),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum GlobalAction {
    Move(Direction),
    Forward,
    Backward,
    Confirm,
    Cancel,
    Text(Text),
    ToggleFullscreen,
    RequestChangeScreenToDeviceSelection,
    RequestChangeScreenToSongEditor,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Action {
    Global(GlobalAction),
    SongScreen(screen::song_editor::Action),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Text {
    WriteDataAtCursor(char),
    RemoveCharAtCursor,
    MoveCursorLeft,
    MoveCursorRight,
}

pub trait HandleAction<InternalAction> {
    fn update(&mut self, app: &Tracky, action: InternalAction, event_tx: EventSender);
    fn input_type(&self) -> InputType;
}
