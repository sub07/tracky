use std::{
    num::NonZeroU32,
    sync::{
        mpsc::{channel, Sender},
        Arc,
    },
};

use log::{error, warn};
use ratatui::{style::Color, Terminal};
use ratatui_wgpu::WgpuBackend;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

use crate::{
    audio::{
        device,
        player::{AudioPlayer, AudioPlayerBuilder},
        Device,
    },
    event::Event,
    keybindings::{InputContext, KeyBindings},
    model::{self},
    view::{popup::Popup, render_root},
    EventSender,
};

pub struct AudioState {
    pub player: AudioPlayer,
    pub state_event_tx: Sender<model::Event>,
}

pub struct Tracky {
    pub state: model::State,
    pub keybindings: KeyBindings,
    pub selected_output_device: Option<Device>,
    pub popup_state: Vec<Popup>,
    pub loader_count: usize,
    pub audio_state: Option<AudioState>,
}

impl Default for Tracky {
    fn default() -> Self {
        Self {
            state: Default::default(),
            keybindings: Default::default(),
            selected_output_device: None,
            popup_state: Vec::new(),
            audio_state: None,
            loader_count: 0,
        }
    }
}

impl Tracky {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input_context(&self) -> crate::keybindings::InputContext {
        if let Some(popup) = self.popup_state.last() {
            popup.input_context()
        } else {
            self.state.patterns.current_input_context()
        }
    }

    pub fn close_popup(&mut self) {
        self.popup_state.pop();
    }

    pub fn send_player_state_event(&self, event: model::Event) {
        if let Some(audio_state) = self.audio_state.as_ref() {
            audio_state.state_event_tx.send(event).unwrap();
        } else {
            warn!("Tried to send event to unloaded player")
        }
    }

    pub fn start_audio_player(&mut self, event_tx: EventSender) {
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
