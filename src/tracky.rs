use crate::{
    audio::{self, device::Device, player::Player, signal::StereoSignal},
    keybindings::{InputContext, KeyBindings},
    log::{DebugLogExt, DisplayLogExt},
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
        if let Some(Device(ref name, ref device)) = self.selected_output_device {
            let mut player = Player::with_device(device.clone()).unwrap();
            let signal = StereoSignal::from_path("assets/piano.wav").unwrap();
            player.queue_signal(&signal);
            player.play().unwrap();
            name.info("Play");
        } else {
            "No device selected".error("Play");
        }
    }
}
