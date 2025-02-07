use std::{fmt, time::Duration};

use crate::{audio::signal, model::channel::Channel};

#[derive(Clone)]
pub struct SongPlayback {
    pub channels: Vec<Channel>,
    pub step_signal: signal::stereo::Owned,
    pub line_signal: signal::stereo::Owned,
    pub current_line: usize,
    pub current_line_duration: Duration,
    pub line_duration: Duration,
    pub last_step_computed_sample_count: usize,
}

impl fmt::Debug for SongPlayback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SongPlayback")
            .field("channels", &self.channels)
            .field("step_signal", &"...")
            .field("line_signal", &"...")
            .field("current_line", &self.current_line)
            .field("current_line_duration", &self.current_line_duration)
            .field("line_duration", &self.line_duration)
            .field(
                "last_step_computed_sample_count",
                &self.last_step_computed_sample_count,
            )
            .finish()
    }
}
