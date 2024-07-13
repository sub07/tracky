use std::time::Duration;

use crate::{
    audio::{
        self,
        frame::CollectFrame,
        player::Player,
        synthesis::{SawWave, SineWave, SquareWave},
    },
    keybindings::{InputContext, KeyBindings},
    log::DebugLogExt,
    model::pattern::Patterns,
    view::popup::Popup,
    DEBUG,
};

pub struct Tracky {
    pub running: bool,
    pub patterns: Patterns,
    pub display_log_console: bool,
    pub keybindings: KeyBindings,
    pub selected_output_device: Option<audio::device::Device>,
    pub popup_state: Option<Popup>,
    pub player: Option<Player>,
}

impl Default for Tracky {
    fn default() -> Self {
        Self {
            running: true,
            patterns: Default::default(),
            display_log_console: DEBUG,
            keybindings: Default::default(),
            selected_output_device: None,
            popup_state: None,
            player: None,
        }
    }
}

impl Tracky {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn exit(&mut self) {
        self.running = false;
    }

    pub fn input_context(&self) -> crate::keybindings::InputContext {
        if self.popup_state.is_some() {
            InputContext::Popup
        } else {
            self.patterns.current_input_context()
        }
    }

    pub fn close_popup(&mut self) {
        self.popup_state = None;
    }

    pub fn play(&mut self) {
        if let Some(ref device) = self.selected_output_device {
            let mut player = Player::with_device(device.clone()).unwrap();
            let signal = SineWave
                .collect_for_duration(
                    Duration::from_secs(1),
                    440.0,
                    0.5.into(),
                    1.0.into(),
                    &mut 0.0,
                    player.sample_rate,
                )
                .append_signal(&SawWave.collect_for_duration(
                    Duration::from_secs(1),
                    440.0,
                    0.5.into(),
                    1.0.into(),
                    &mut 0.0,
                    player.sample_rate,
                ))
                .unwrap()
                .append_signal(&SquareWave.collect_for_duration(
                    Duration::from_secs(3),
                    440.0,
                    0.5.into(),
                    1.0.into(),
                    &mut 0.0,
                    player.sample_rate,
                ))
                .unwrap();
            player.queue_signal(&signal);
            player.play().unwrap();
            self.player = Some(player);
        } else {
            "No device selected".error("Play");
        }
    }
}
