use std::sync::mpsc::channel;
use std::{env, io, thread};

use ::log::{error, info};
use audio::Hosts;
use event::{Action, AsyncAction, Event};
use log::write_logs_to_file;
use model::pattern::NoteName;
use model::song;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::Terminal;
use tracky::Tracky;
use tui::Tui;
use view::popup::Popup;

mod audio;
mod event;
mod keybindings;
mod log;
mod model;
mod service;
mod tracky;
mod tui;
mod view;

#[cfg(debug_assertions)]
const DEBUG: bool = true;

#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    env::set_var("RUST_BACKTRACE", "1");

    log::setup()?;

    let mut app = Tracky::new();
    app.song.handle_event(song::Event::SetNoteField {
        note: NoteName::A,
        octave_modifier: 0,
    });

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.init()?;

    let (event_tx, event_rx) = channel();
    let input_thread_event_tx = event_tx.clone();
    thread::spawn(move || loop {
        match ratatui::crossterm::event::read() {
            Ok(ratatui::crossterm::event::Event::Key(key_event))
                if key_event.kind == KeyEventKind::Press =>
            {
                input_thread_event_tx.send(Event::Key(key_event)).unwrap();
            }
            Ok(ratatui::crossterm::event::Event::Resize(w, h)) => {
                input_thread_event_tx
                    .send(Event::Resize {
                        width: w,
                        height: h,
                    })
                    .unwrap();
            }
            Err(err) => input_thread_event_tx
                .send(Event::Panic(err.into()))
                .unwrap(),
            _ => {}
        }
    });

    while app.running {
        tui.draw(&mut app)?;
        let mut event = event_rx.recv()?;
        // debug!("{event:?}");
        if let Some(popup) = &mut app.popup_state {
            if let Some(unprocessed_event) = popup.handle_event(event, event_tx.clone()) {
                event = unprocessed_event;
            } else {
                continue;
            }
        }
        match event {
            Event::Key(key_event) => {
                if let Some(event) = app.keybindings.action(key_event.code, app.input_context()) {
                    event_tx.send(event).unwrap();
                }
            }
            Event::Action(action) => match action {
                Action::TogglePlay => {}
                Action::WriteLogsOnDisk => {
                    if let Err(e) = write_logs_to_file("tracky.log") {
                        error!("Could not write log: {e}");
                    }
                }
                Action::ClearLogsPanel => crate::log::clear_entries(),
                Action::ToggleLogsPanel => app.display_log_console = !app.display_log_console,
                Action::Cancel | Action::Confirm => {}
                Action::RequestOpenDeviceSelectionPopup => {
                    event_tx.send(Event::StartLoading).unwrap();
                    let event_tx_clone = event_tx.clone();
                    thread::spawn(move || {
                        event_tx_clone
                            .send(Event::LoadingDone(AsyncAction::OpenDeviceSelectionPopup(
                                Hosts::load(),
                            )))
                            .unwrap();
                    });
                }
                Action::Move(direction) => event_tx
                    .send(Event::Song(song::Event::MoveCursor(direction)))
                    .unwrap(),
                Action::Forward => todo!(),
                Action::Backward => todo!(),
            },
            Event::Panic(error) => {
                panic!("{error:?}");
            }
            Event::Composite(events) => {
                for event in events {
                    event_tx.send(event).unwrap();
                }
            }
            Event::Resize { width, height } => info!("{width}x{height}"),
            Event::AsyncAction(async_action) => match async_action {
                event::AsyncAction::OpenDeviceSelectionPopup(hosts) => {
                    app.popup_state = Some(Popup::AudioDeviceSelection(hosts.into()));
                }
            },
            Event::StartLoading => app.loader_count += 1,
            Event::LoadingDone(async_action) => {
                app.loader_count = app.loader_count.saturating_sub(1);
                event_tx.send(Event::AsyncAction(async_action)).unwrap();
            }
            Event::ClosePopup => app.close_popup(),
            Event::SetPlayingDevice(device) => app.selected_output_device = Some(device),
            Event::Song(event) => app.song.handle_event(event),
            Event::ExitApp => app.exit(),
        }
    }

    tui.exit()?;

    Ok(())
}
