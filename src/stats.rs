use std::collections::{hash_map, HashMap};

use itertools::Itertools;

use crate::{event::Event, model};

pub struct Statistics {
    update_per_second: f32,
    frame_acc: u64,
    dt_acc: chrono::Duration,
    last_time: chrono::NaiveDateTime,
    frame_agreggator_len: u64,
    update_event_histogram: HashMap<String, u64>,
}

impl Default for Statistics {
    fn default() -> Self {
        Self::new(50)
    }
}

impl Statistics {
    pub fn new(frame_agreggator_len: u64) -> Statistics {
        Statistics {
            update_per_second: 0.0,
            frame_acc: 0,
            dt_acc: chrono::Duration::zero(),
            last_time: chrono::Utc::now().naive_utc(),
            frame_agreggator_len,
            update_event_histogram: HashMap::new(),
        }
    }

    pub fn start_frame_with_event(&mut self, event: &Event) {
        let command_str_mapper = |command: &model::Command| match command {
            model::Command::ChangeGlobalOctave { increment: _ } => {
                String::from("ChangeGlobalOctave")
            }
            model::Command::ChangeSelectedInstrument { increment: _ } => {
                String::from("ChangeSelectedInstrument")
            }
            model::Command::SetNoteField {
                note: _,
                octave_modifier: _,
            } => String::from("SetNoteField"),
            model::Command::MoveCursor(_) => String::from("MoveCursor"),
            model::Command::SetNoteFieldToCut => String::from("SetNoteFieldToCut"),
            model::Command::ClearField => String::from("ClearField"),
            model::Command::SetOctaveField(_) => String::from("SetOctaveField"),
            model::Command::SetHexField(_) => String::from("SetHexField"),
            model::Command::CreateNewPattern => String::from("CreateNewPattern"),
            model::Command::GoToNextPattern => String::from("GoToNextPattern"),
            model::Command::GoToPreviousPattern => String::from("GoToPreviousPattern"),
            model::Command::StartSongPlaybackFromBeginning => {
                String::from("StartSongPlaybackFromBeginning")
            }
            model::Command::StopSongPlayback => String::from("StopSongPlayback"),
            model::Command::InitializeAudio { frame_rate: _ } => String::from("InitializeAudio"),
            model::Command::UpdatePlaybackSampleCount(_) => {
                String::from("UpdatePlaybackSampleCount")
            }
            model::Command::PerformPlaybacksStep => String::from("PerformPlaybacksStep"),
            model::Command::ClearChannels => String::from("ClearChannels"),
        };
        let event_str = match event {
            Event::KeyPressed(_) => String::from("KeyPressed"),
            Event::Text(_) => String::from("Text"),
            Event::State(c) => format!("State({})", command_str_mapper(c)),
            Event::AudioCallback(c) => format!("AudioCallback({})", command_str_mapper(c)),
            Event::Panic(_) => String::from("Panic"),
            Event::Action(_) => String::from("Action"),
            Event::AsyncAction(_) => String::from("AsyncAction"),
            Event::Resize { .. } => String::from("Resize"),
            Event::Composite(_) => String::from("Composite"),
            Event::StartLoading => String::from("StartLoading"),
            Event::LoadingDone(_) => String::from("LoadingDone"),
            Event::ClosePopup => String::from("ClosePopup"),
            Event::SetPlayingDevice(_) => String::from("SetPlayingDevice"),
            Event::StartAudioPlayer => String::from("StartAudioPlayer"),
            Event::StopAudioPlayer(_) => String::from("StopAudioPlayer"),
            Event::RequestRedraw => String::from("RequestRedraw"),
            Event::TextSubmitted(_, _) => String::from("TextSubmitted"),
            Event::ExitApp => String::from("ExitApp"),
        };
        match self.update_event_histogram.entry(event_str) {
            hash_map::Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() = occupied_entry.get() + 1;
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        let now = chrono::Utc::now().naive_utc();
        self.dt_acc += now - self.last_time;
        self.frame_acc += 1;
        if self.frame_acc == self.frame_agreggator_len {
            self.update_per_second = 1.0 / (self.dt_acc.as_seconds_f32() / self.frame_acc as f32);
            self.dt_acc = chrono::Duration::zero();
            self.frame_acc = 0;
        }
        self.last_time = now;
    }

    pub fn update_per_second(&self) -> f32 {
        self.update_per_second
    }

    pub fn update_event_histogram(&self) -> impl Iterator<Item = (&str, u64)> {
        self.update_event_histogram
            .iter()
            .sorted_by_key(|(_, count)| *count)
            .map(|(event, count)| (event.as_str(), *count))
            .rev()
    }
}
