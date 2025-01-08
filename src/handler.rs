use std::{
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

use crate::{
    audio::{
        player::{AudioPlayerBuilder, SineState},
        Hosts,
    },
    event::{AsyncAction, Event},
    keybindings::Action,
    log::{clear_entries, write_logs_to_file},
    tracky::{Playback, Tracky},
};
use log::error;

pub fn handle_action(
    action: Action,
    app: &mut Tracky,
    event_tx: Sender<Event>,
) -> anyhow::Result<()> {
    match action {
        Action::Note {
            note_name,
            octave_modifier,
        } => app.patterns.set_note(note_name, octave_modifier),
        Action::Hex(digit) => app.patterns.set_hex(digit),
        Action::Octave(octave) => app.patterns.set_octave(octave),
        Action::ClearField => app.patterns.clear(),
        Action::Move(direction, step) => app.patterns.move_cursor(direction, step),
        Action::InsertPattern => todo!(),
        Action::NextPattern => todo!(),
        Action::PreviousPattern => todo!(),
        Action::TogglePlay => {
            if let Some(_) = app.playback_state.take() {
            } else {
                let (state_event_tx, state_event_rx) = channel();
                app.playback_state = Some(Playback {
                    player: AudioPlayerBuilder::new()
                        .name("Sine player")
                        .device(app.selected_output_device.clone())
                        .initial_state(SineState {
                            freq: 440.0,
                            phase: 0.0,
                            time_played: Duration::ZERO,
                        })
                        .event_tx(event_tx.clone())
                        .state_event_rx(state_event_rx)
                        .build()
                        .into_player()?,
                    state_event_tx,
                })
            }
        }
        Action::NoteCut => app.patterns.set_note_cut(),
        Action::ModifyDefaultOctave(i) => app.patterns.modify_default_octave(i),
        Action::ExitApp => app.exit(),
        Action::WriteLogsOnDisk => {
            if let Err(e) = write_logs_to_file("tracky.log") {
                error!("Could not write log: {e}");
            }
        }
        Action::ClearLogsPanel => clear_entries(),
        Action::ToggleLogsPanel => app.display_log_console = !app.display_log_console,
        Action::Cancel | Action::Confirm => {}
        Action::RequestOpenDeviceSelectionPopup => {
            event_tx.send(Event::StartLoading).unwrap();
            thread::spawn(move || {
                event_tx
                    .send(Event::LoadingDone(AsyncAction::OpenDeviceSelectionPopup(
                        Hosts::load(),
                    )))
                    .unwrap();
            });
        }
    }

    Ok(())
}
