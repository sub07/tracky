use std::{
    collections::{hash_map, HashMap},
    time::Instant,
};

use itertools::Itertools;

use crate::{event::Event, model};

pub struct Rate {
    rate: f32,
    acc: u64,
    last_time: Instant,
    global_rate_acc: f32,
    global_rate_count: u64,
    rate_smoothing_len: u64,
}

impl Rate {
    fn new(rate_smoothing_len: u64) -> Rate {
        Rate {
            rate: 0.0,
            acc: 0,
            last_time: Instant::now(),
            global_rate_acc: 0.0,
            global_rate_count: 0,
            rate_smoothing_len,
        }
    }

    fn update(&mut self) {
        self.acc += 1;
        if self.acc == self.rate_smoothing_len {
            let now = Instant::now();
            let since_last = now - self.last_time;
            self.rate = 1.0 / (since_last.as_secs_f32() / self.acc as f32);
            self.global_rate_acc += self.rate;
            self.global_rate_count += 1;
            self.acc = 0;
            self.last_time = now;
        }
    }

    /// Smoothed rate over the last `rate_smoothing_len` frames.
    pub fn rate(&self) -> f32 {
        self.rate
    }

    /// Smoothed rate over the whole app run time.
    pub fn global_rate(&self) -> f32 {
        self.global_rate_acc / self.global_rate_count as f32
    }
}

// TODO: Add heap allocation count from audio callback counter (need channel to send message to stats)
pub struct Statistics {
    pub update_rate: Rate,
    pub render_rate: Rate,
    update_event_histogram: HashMap<String, u64>,
    state_time: Instant,
}

impl Default for Statistics {
    fn default() -> Self {
        Self::new(10)
    }
}

impl Statistics {
    pub fn new(rate_smoothing_len: u64) -> Statistics {
        Statistics {
            update_rate: Rate::new(rate_smoothing_len),
            render_rate: Rate::new(rate_smoothing_len),
            update_event_histogram: HashMap::new(),
            state_time: Instant::now(),
        }
    }

    pub fn record_event(&mut self, event: &Event) {
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
            model::Command::SetNoteCut => String::from("SetNoteCut"),
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
            model::Command::ChangeGlobalVolume { .. } => String::from("ChangeGlobalVolume"),
        };
        let event_str = match event {
            Event::KeyPressed(_, _) => String::from("KeyPressed"),
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
            Event::ExitApp => String::from("ExitApp"),
            Event::ChangeScreen(screen) => format!(
                "ChangeScreen({})",
                match screen {
                    crate::view::screen::Screen::DeviceSelection(_) => {
                        "DeviceSelection"
                    }
                    crate::view::screen::Screen::SongEditor(_) => "SongEditor",
                }
            ),
        };
        match self.update_event_histogram.entry(event_str) {
            hash_map::Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() = occupied_entry.get() + 1;
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        self.update_rate.update();
    }

    pub fn record_render(&mut self) {
        self.render_rate.update();
    }

    pub fn update_event_histogram(&self) -> impl Iterator<Item = (&str, u64)> {
        self.update_event_histogram
            .iter()
            .sorted_by_key(|(_, count)| *count)
            .map(|(event, count)| (event.as_str(), *count))
            .rev()
    }

    pub fn print_stats(&self) {
        let now = Instant::now();
        println!("================ Tracky stats ================");
        println!("App ran for {:?}", now - self.state_time);
        println!(
            "Average render per second: {}",
            self.render_rate.global_rate()
        );
        println!(
            "Average update per second: {}",
            self.update_rate.global_rate()
        );
        let total_event_count = self.update_event_histogram.values().sum::<u64>();
        println!("{total_event_count} events fired:");
        for (event, count) in self.update_event_histogram().collect_vec() {
            println!("\t{}: {}", event, count);
        }
    }
}
