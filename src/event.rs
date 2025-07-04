use winit::{event::KeyEvent, keyboard::ModifiersState};

use crate::{
    audio::device::{ConfiguredDevice, Devices},
    keybindings::InputContext,
    model::{
        self,
        pattern::{HexDigit, NoteFieldValue, NoteName, OctaveValue},
    },
    utils::Direction,
    view::screen::Screen,
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
    ExitApp,
}

#[derive(Debug)]
pub enum AsyncAction {
    GetDevices(Devices),
}

#[derive(Debug, Clone)]
pub enum Action {
    Move(Direction),
    Forward,
    Backward,
    Confirm,
    Cancel,
    TogglePlay,
    ToggleFullscreen,
    RequestChangeScreenToDeviceSelection,
    RequestChangeScreenToSongEditor,
    ShowGlobalVolumePopup,
    KillNotes,
    ChangeGlobalOctave {
        increment: i32,
    },
    ChangeSelectedInstrument {
        increment: i32,
    },
    SetNoteField {
        note: NoteName,
        octave_modifier: i32,
    },
    SetNoteCut,
    ClearField,
    SetOctaveField(OctaveValue),
    SetHexField(HexDigit),
    CreateNewPattern,
    GoToNextPattern,
    GoToPreviousPattern,
    Text(Text),
}

#[derive(Debug, Clone)]
pub enum Text {
    WriteDataAtCursor(char),
    RemoveCharAtCursor,
    MoveCursorLeft,
    MoveCursorRight,
}

pub trait HandleAction<InternalAction> {
    fn map_action(&self, action: &Action) -> Option<InternalAction>;
    fn update(&mut self, event: InternalAction, event_tx: EventSender);
    fn input_context(&self) -> InputContext;
    fn handle_action(&mut self, action: Action, event_tx: EventSender) {
        if let Some(popup_event) = self.map_action(&action) {
            self.update(popup_event, event_tx);
        }
    }
}
