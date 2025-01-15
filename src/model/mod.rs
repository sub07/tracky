use joy_vector::Vector;
use pattern::{HexDigit, NoteName, OctaveValue, Patterns};
use playback::song::SongPlayback;

use crate::utils::Direction;

pub mod channel;
pub mod midi;
pub mod pattern;
pub mod playback;

#[derive(Clone)]
pub struct State {
    pub patterns: Patterns,
    pub global_octave: OctaveValue,
    pub line_per_second: f32,
    pub playback: Option<SongPlayback>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            patterns: Default::default(),
            global_octave: Default::default(),
            line_per_second: 16.0,
            playback: None,
        }
    }
}

impl State {
    pub fn is_playing(&self) -> bool {
        self.playback.is_some()
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    MutateGlobalOctave {
        increment: i32,
    },
    SetNoteField {
        note: NoteName,
        octave_modifier: i32,
    },
    MoveCursor(Direction),
    SetNoteFieldToCut,
    ClearField,
    SetOctaveField(OctaveValue),
    SetHexField(HexDigit),
    NewPattern,
    NextPattern,
    PreviousPattern,
    StartSongPlayback {
        frame_rate: f32,
    },
    StopSongPlayback,
    UpdatePlaybackSampleCount(usize),
}
