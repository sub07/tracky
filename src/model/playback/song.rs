use std::{fmt, time::Duration};

use crate::audio::signal;

#[derive(Clone)]
pub struct Playback {
    pub line_signal: signal::stereo::Owned,
    pub current_line: usize,
    pub current_line_duration: Duration,
    pub line_duration: Duration,
    pub is_playing: bool,
}

impl fmt::Debug for Playback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SongPlayback")
            .field("step_signal", &"...")
            .field("line_signal", &"...")
            .field("current_line", &self.current_line)
            .field("current_line_duration", &self.current_line_duration)
            .field("line_duration", &self.line_duration)
            .finish()
    }
}
