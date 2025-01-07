use crate::{
    audio::Device,
    keybindings::{InputContext, KeyBindings},
    model::pattern::Patterns,
    // service::playback::Playback,
    view::popup::Popup,
    DEBUG,
};

pub struct Tracky {
    pub running: bool,
    pub patterns: Patterns,
    pub display_log_console: bool,
    pub keybindings: KeyBindings,
    pub selected_output_device: Option<Device>,
    pub popup_state: Option<Popup>,
    pub line_per_second: f32,
    pub loader_count: usize,
    // pub playback_state: Option<Playback>,
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
            // playback_state: None,
            line_per_second: 16.0,
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
            self.patterns.current_input_context()
        }
    }

    pub fn close_popup(&mut self) {
        self.popup_state = None;
    }
}
