use std::{
    cell::{Ref, RefCell, RefMut},
    ops::Deref,
    rc::Rc,
    sync::{
        mpsc::{channel, Sender},
        Arc, RwLock, RwLockReadGuard, RwLockWriteGuard,
    },
};

use log::error;

use crate::{
    audio::{
        device::ConfiguredDevice,
        player::{AudioPlayer, AudioPlayerBuilder},
    },
    event::{Action, Event},
    keybindings::Keybindings,
    model::{self, Command},
    stats::Statistics,
    view::{
        popup::Popup,
        screen::{self, Screen},
    },
    EventSender,
};

pub struct AudioState {
    pub _player: AudioPlayer,
    pub state_event_tx: Sender<model::Command>,
}

#[derive(Default)]
pub struct Tracky {
    pub state: model::State,
    pub keybindings: Keybindings,
    pub selected_output_device: Option<ConfiguredDevice>,
    pub current_popup: Option<Popup>,
    pub current_screen: Screen,
    pub loader_count: usize,
    pub audio_state: Option<AudioState>,
    pub stats: Statistics,
}

impl Tracky {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn input_type(&self) -> crate::keybindings::InputType {
        self.current_popup
            .as_ref()
            .map(Popup::input_context)
            .unwrap_or(self.current_screen.input_location())
    }

    pub fn open_popup(&mut self, popup: Popup) {
        self.current_popup = Some(popup);
    }

    pub fn close_popup(&mut self) {
        self.current_popup = None;
    }

    pub fn change_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn send_player_state_event(&self, event: model::Command) {
        if let Some(audio_state) = self.audio_state.as_ref() {
            audio_state.state_event_tx.send(event).unwrap();
        }
    }

    pub fn update_screen(&mut self, action: Action, event_tx: EventSender) {
        self.current_screen.update(self, action, event_tx);
    }

    pub fn start_audio_player(&mut self, event_tx: EventSender) {
        if let Some(selected_output_device) = self.selected_output_device.clone() {
            let (state_event_tx, state_event_rx) = channel();
            match AudioPlayerBuilder::new()
                .device(selected_output_device)
                .event_tx(event_tx.clone())
                .initial_state(self.state.clone())
                .state_event_rx(state_event_rx)
                .build()
                .into_player()
            {
                Ok(player) => {
                    event_tx
                        .send_event(Event::State(Command::InitializeAudio {
                            frame_rate: player.frame_rate,
                        }))
                        .unwrap();
                    self.audio_state = Some(AudioState {
                        _player: player,
                        state_event_tx,
                    });
                }
                Err(error) => error!("{error}"),
            }
        }
    }

    pub fn stop_audio_player(&mut self) {
        self.audio_state = None;
    }

    pub fn teardown(&mut self) {
        self.stop_audio_player();
    }
}
