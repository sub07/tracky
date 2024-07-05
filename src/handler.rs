use crate::{
    keybindings::Action,
    log::{clear_entries, DebugLogExt},
    tracky::Tracky,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub async fn handle_key_events(key_event: KeyEvent, app: &mut Tracky) -> eyre::Result<()> {
    if let Some(action) = app.keybindings.action(
        key_event.modifiers,
        key_event.code,
        app.patterns.current_input_context(),
    ) {
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
            Action::TogglePlay => todo!(),
            Action::NoteCut => app.patterns.set_note_cut(),
            Action::ModifyDefaultOctave(i) => app.patterns.modify_default_octave(i),
            Action::Exit => app.exit(),
        }
    } else {
        match key_event.code {
            KeyCode::F(9) => {
                "Save on Disk".error("not implemented");
            }
            KeyCode::F(10) => clear_entries(),
            KeyCode::F(12) => app.display_log_console = !app.display_log_console,
            _ => {}
        }
    }
    Ok(())
}
