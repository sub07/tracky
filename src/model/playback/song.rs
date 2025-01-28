use std::time::Duration;

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
