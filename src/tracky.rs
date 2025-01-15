use std::sync::mpsc::{channel, Sender};

use log::{error, warn};

use crate::{
    audio::{
        player::{AudioPlayer, AudioPlayerBuilder},
        Device,
    },
    event::Event,
    keybindings::{InputContext, KeyBindings},
    model::{self},
    view::popup::Popup,
    DEBUG,
};

pub struct AudioState {
    pub player: AudioPlayer,
    pub state_event_tx: Sender<model::Event>,
}

pub struct Tracky {
    pub running: bool,
    pub state: model::State,
    pub display_log_console: bool,
    pub keybindings: KeyBindings,
    pub selected_output_device: Option<Device>,
    pub popup_state: Option<Popup>,
    pub loader_count: usize,
    pub audio_state: Option<AudioState>,
}

impl Default for Tracky {
    fn default() -> Self {
        Self {
            running: true,
            state: Default::default(),
            display_log_console: DEBUG,
            keybindings: Default::default(),
            selected_output_device: None,
            popup_state: None,
            audio_state: None,
            loader_count: 0,
        }
    }
}

impl Tracky {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exit(&mut self) {
        self.running = false;
    }

    pub fn input_context(&self) -> crate::keybindings::InputContext {
        if self.popup_state.is_some() {
            InputContext::Popup
        } else {
            self.state.patterns.current_input_context()
        }
    }

    pub fn close_popup(&mut self) {
        self.popup_state = None;
    }

    pub fn send_player_state_event(&self, event: model::Event) {
        if let Some(audio_state) = self.audio_state.as_ref() {
            audio_state.state_event_tx.send(event).unwrap();
        } else {
            warn!("Tried to send event to unloaded player")
        }
    }

    pub fn start_audio_player(&mut self, event_tx: Sender<Event>) {
        let (state_event_tx, state_event_rx) = channel();
        match AudioPlayerBuilder::new()
            .device(self.selected_output_device.clone())
            .event_tx(event_tx)
            .initial_state(self.state.clone())
            .state_event_rx(state_event_rx)
            .build()
            .into_player()
        {
            Ok(player) => {
                self.audio_state = Some(AudioState {
                    player,
                    state_event_tx,
                })
            }
            Err(error) => error!("{error}"),
        }
    }

    pub fn stop_audio_player(&mut self) {
        self.audio_state = None;
    }
}
