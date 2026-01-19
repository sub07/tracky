use std::sync::mpsc::{Sender, channel};

use log::error;

pub mod view;

use crate::{
    EventSender,
    app::view::{popup::Popup, screen::Screen},
    audio::{
        device::ConfiguredDevice,
        player::{AudioPlayer, AudioPlayerBuilder},
    },
    event::{Action, Event},
    keybindings::Keybindings,
    model::{self, Command},
    stats::Statistics,
};

pub struct AudioState {
    pub _player: AudioPlayer,
    pub state_event_tx: Sender<model::Command>,
}

pub struct Tracky {
    pub state: model::State,
    pub keybindings: Keybindings,
    pub selected_output_device: Option<ConfiguredDevice>,
    pub current_popup: Option<Popup>,
    pub current_screen: Screen,
    pub loader_count: usize,
    pub audio_state: Option<AudioState>,
    pub stats: Statistics,
    pub event_sender: EventSender,
}

impl Tracky {
    pub fn new(event_sender: EventSender) -> Self {
        Self {
            state: Default::default(),
            keybindings: Default::default(),
            selected_output_device: Default::default(),
            current_popup: Default::default(),
            current_screen: Default::default(),
            loader_count: Default::default(),
            audio_state: Default::default(),
            stats: Default::default(),
            event_sender,
        }
    }

    pub fn input_context(&self) -> crate::keybindings::InputContext {
        self.popup_input_context()
            .unwrap_or_else(|| self.screen_input_context())
    }

    pub fn handle_action(&mut self, action: Action) {
        if let Some(action) = self.handle_action_on_popup(action) {
            self.handle_action_on_screen(action);
        }
    }

    pub fn send_player_state_event(&self, event: model::Command) {
        if let Some(audio_state) = self.audio_state.as_ref() {
            audio_state.state_event_tx.send(event).unwrap();
        }
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
