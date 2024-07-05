use crate::{keybindings::KeyBindings, model::pattern::Patterns};

pub struct Tracky {
    pub running: bool,
    pub patterns: Patterns,
    pub display_log_console: bool,
    pub keybindings: KeyBindings,
}

impl Default for Tracky {
    fn default() -> Self {
        Self {
            running: true,
            patterns: Default::default(),
            #[cfg(debug_assertions)]
            display_log_console: true,
            #[cfg(not(debug_assertions))]
            display_log_console: false,
            keybindings: Default::default(),
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
}
