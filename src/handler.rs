use std::{
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

use crate::{
    audio::Hosts,
    event::{AsyncEvent, Event},
    keybindings::Action,
    log::{clear_entries, write_logs_to_file},
    tracky::Tracky,
    view::popup::{self, Popup},
};
use cpal::Devices;
use joy_impl_ignore::{debug::DebugImplIgnore, eq::PartialEqImplIgnore};
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
        Action::TogglePlay => {}
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
        Action::SetPlayingDevice(DebugImplIgnore(PartialEqImplIgnore(device))) => {
            app.selected_output_device = Some(device)
        }
        Action::Cancel | Action::Confirm => {}
        Action::ClosePopup => app.close_popup(),
        Action::OpenDeviceSelectionPopup => {
            let (devices_tx, devices_rx) = channel();
            app.popup_state = Some(Popup::AudioDeviceSelection(
                popup::audio_device_selection::Popup::Loading(devices_rx),
            ));
            thread::spawn(move || {
                devices_tx.send(Hosts::load()).unwrap();
                event_tx
                    .send(Event::Async(AsyncEvent::LoadingDone))
                    .unwrap();
            });
        }
    }

    Ok(())
}
