use winit::event::KeyEvent;

use crate::{
    audio::device::{ConfiguredDevice, Devices},
    keybindings::InputContext,
    model::{self},
    utils::Direction,
    view::popup::input::InputId,
    EventSender,
};

#[derive(Debug)]
pub enum Event {
    KeyPressed(KeyEvent),
    Text(Text),
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
    TextSubmitted(InputId, String),
    ExitApp,
}

#[derive(Debug)]
pub enum AsyncAction {
    OpenDeviceSelectionPopup(Devices),
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
    KillNotes,
    ChangeSelectedInstrument { increment: i32 },
}

#[derive(Debug, Clone)]
pub enum Text {
    WriteDataAtCursor(char),
    RemoveCharAtCursor,
    MoveCursorLeft,
    MoveCursorRight,
}

pub trait EventAware<InternalEvent> {
    fn map_event(&self, event: &Event) -> Option<InternalEvent>;
    fn update(&mut self, event: InternalEvent, event_tx: EventSender);
    fn input_context(&self) -> InputContext;
    fn handle_event(&mut self, event: Event, event_tx: EventSender) -> Option<Event> {
        if let Some(popup_event) = self.map_event(&event) {
            self.update(popup_event, event_tx);
            None
        } else {
            Some(event)
        }
    }
}
