use std::sync::mpsc::Sender;

use crate::{
    event::Event,
    keybindings::Action,
    log::{clear_entries, write_logs_to_file},
    tracky::Tracky,
    view::popup::Popup,
};
use joy_impl_ignore::{debug::DebugImplIgnore, eq::PartialEqImplIgnore};
use log::error;

pub fn handle_action(
    mut action: Action,
    app: &mut Tracky,
    event_tx: Sender<Event>,
) -> anyhow::Result<()> {
    if let Some(ref mut popup) = app.popup_state {
        let mut is_action_consumed = true;
        if let Some(popup_action) = popup.handle_action(action.clone(), &mut is_action_consumed) {
            action = popup_action;
        } else if is_action_consumed {
            return Ok(());
        }
    }
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
        Action::Composite(actions) => {
            for action in actions.into_iter() {
                handle_action(action, app, event_tx.clone())?;
            }
        }
        Action::OpenDeviceSelectionPopup => {
            app.popup_state = Some(Popup::AudioDeviceSelection(Default::default()))
        }
    }

    Ok(())
}
