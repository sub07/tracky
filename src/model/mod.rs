use anyhow::anyhow;
use channel::Channel;
use instrument::Instruments;
use pattern::{HexDigit, NoteName, OctaveValue, Patterns};
use playback::song;

use crate::{
    audio::{signal, Volume},
    utils::Direction,
};

pub mod channel;
pub mod instrument;
pub mod midi;
pub mod pattern;
pub mod playback;

#[derive(Clone, Debug)]
pub struct State {
    pub patterns: Patterns,
    pub channels: Vec<Channel>,

    pub global_octave: OctaveValue,
    pub global_volume: Volume,
    pub line_per_second: f32,

    pub follow_playing: bool,

    pub step_output: Option<signal::stereo::Owned>,
    pub computed_frame_count: usize,

    pub song_playback: Option<song::Playback>,

    pub instruments: Instruments,
}

impl Default for State {
    fn default() -> Self {
        let patterns = Patterns::default();
        Self {
            channels: vec![Channel::new(); patterns.channel_count as usize],
            step_output: None,
            global_octave: Default::default(),
            line_per_second: 16.0,
            global_volume: Volume::new_unchecked(0.3),
            song_playback: None,
            instruments: Default::default(),
            follow_playing: false,
            patterns,
            computed_frame_count: 0,
        }
    }
}

impl State {
    pub fn is_song_playing(&self) -> bool {
        self.song_playback
            .as_ref()
            .is_some_and(|playback| playback.is_playing)
    }

    pub fn currently_played_line(&self) -> Option<usize> {
        self.song_playback
            .as_ref()
            .map(|playback| playback.current_line)
    }

    pub fn output_samples(&self) -> anyhow::Result<signal::stereo::Ref> {
        self.step_output
            .as_ref()
            .ok_or_else(|| anyhow!("Uninitialized state"))
            .and_then(|output| output.sub_signal(0, self.computed_frame_count))
    }

    pub fn should_perform_step(&self) -> bool {
        self.song_playback
            .as_ref()
            .is_some_and(|playback| playback.is_playing) // Channel playing should be sufficent but this is needed to play empty patterns
            || self.channels.iter().any(Channel::is_playing)
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    ChangeGlobalOctave {
        increment: i32,
    },
    ChangeSelectedInstrument {
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
    CreateNewPattern,
    GoToNextPattern,
    GoToPreviousPattern,
    StartSongPlaybackFromBeginning,
    StopSongPlayback,
    InitializeAudio {
        frame_rate: f32,
    },
    UpdatePlaybackSampleCount(usize),
    PerformPlaybacksStep,
    ClearChannels,
}
