use log::info;

use crate::{
    app::{
        Tracky,
        view::popup::{Popup, change_volume},
    },
    event::{Action, Event, HandleAction},
    keybindings::InputContext,
    model::Command,
};

pub mod device_selection;
pub mod song_editor;

#[derive(Default, Debug)]
pub enum Screen {
    DeviceSelection(device_selection::State),
    #[default]
    SongEditor,
}

impl Tracky {
    pub fn change_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn screen_input_context(&self) -> InputContext {
        match &self.current_screen {
            Screen::DeviceSelection(state) => state.input_context(),
            Screen::SongEditor => self.state.patterns.input_context(),
        }
    }

    pub fn handle_action_on_screen(&mut self, action: Action) {
        macro_rules! command {
            ($command:expr) => {
                self.event_sender
                    .send_event(crate::event::Event::State($command))
                    .unwrap()
            };
        }

        match &mut self.current_screen {
            Screen::DeviceSelection(state) => {
                state.handle_action(action, self.event_sender.clone());
            }
            Screen::SongEditor => match action {
                Action::TogglePlay => {
                    if self.state.is_song_playing() {
                        command!(Command::StopSongPlayback);
                    } else if self.audio_state.is_some() {
                        command!(Command::StartSongPlaybackFromBeginning);
                    } else {
                        log::warn!("Select a device with F1 to play the song");
                    }
                }
                Action::Move(direction) => {
                    command!(Command::MoveCursor(direction));
                }
                Action::KillNotes => command!(Command::ClearChannels),
                Action::ChangeSelectedInstrument { increment } => {
                    command!(Command::ChangeSelectedInstrument { increment });
                }
                Action::ShowGlobalVolumePopup => {
                    self.open_popup(Popup::ChangeVolume(change_volume::Popup::new(
                        "Global volume",
                        self.state.global_volume.db(),
                        |value, event_sender| {
                            let volume = value.volume();
                            event_sender
                                .send_event(Event::Composite(vec![
                                    Event::State(Command::ChangeGlobalVolume { volume }),
                                    Event::ClosePopup,
                                ]))
                                .unwrap();
                        },
                    )));
                }
                Action::ChangeGlobalOctave { increment } => {
                    command!(Command::ChangeGlobalOctave { increment });
                }
                Action::SetNoteField {
                    note,
                    octave_modifier,
                } => command!(Command::SetNoteField {
                    note,
                    octave_modifier
                }),
                Action::SetNoteCut => command!(Command::SetNoteCut),
                Action::ClearField => command!(Command::ClearField),
                Action::SetOctaveField(octave_value) => {
                    command!(Command::SetOctaveField(octave_value));
                }
                Action::SetHexField(hex_digit) => {
                    command!(Command::SetHexField(hex_digit));
                }
                Action::CreateNewPattern => {
                    command!(Command::CreateNewPattern);
                }
                Action::GoToNextPattern => {
                    command!(Command::GoToNextPattern);
                }
                Action::GoToPreviousPattern => {
                    command!(Command::GoToPreviousPattern);
                }
                action => {
                    info!("Action {action:?} ignored in song editor");
                }
            },
        }
    }
}
