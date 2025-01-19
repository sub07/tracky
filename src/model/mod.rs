use instrument::Instruments;
use pattern::{HexDigit, NoteName, OctaveValue, Patterns};
use playback::song::SongPlayback;

use crate::{audio::Volume, utils::Direction};

pub mod channel;
pub mod instrument;
pub mod midi;
pub mod pattern;
pub mod playback;

#[derive(Clone)]
pub struct State {
    pub patterns: Patterns,
    pub global_octave: OctaveValue,
    pub line_per_second: f32,
    pub global_volume: Volume,
    pub playback: Option<SongPlayback>,
    pub instruments: Instruments,
}

impl Default for State {
    fn default() -> Self {
        Self {
            patterns: Default::default(),
            global_octave: Default::default(),
            line_per_second: 16.0,
            global_volume: Volume::new_unchecked(0.3),
            playback: None,
            instruments: Default::default(),
        }
    }
}

impl State {
    pub fn is_playing(&self) -> bool {
        self.playback.is_some()
    }

    pub fn is_playback_done(&self) -> bool {
        self.playback
            .as_ref()
            .is_some_and(|playback| playback.current_line as i32 >= self.patterns.channel_len)
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
    PerformStepPlayback,
}
