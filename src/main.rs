use std::sync::mpsc::channel;
use std::{env, io, thread};

use ::log::{debug, error, info, warn};
use audio::Hosts;
use event::{Action, AsyncAction, Event};
use log::write_logs_to_file;
use model::pattern::NoteName;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::Terminal;
use tracky::Tracky;
use tui::Tui;
use view::popup::input::InputId;
use view::popup::{self, Popup};

mod audio;
mod event;
mod keybindings;
mod log;
mod model;
mod service;
mod tracky;
mod tui;
mod utils;
mod view;

#[cfg(debug_assertions)]
const DEBUG: bool = true;

#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    env::set_var("RUST_BACKTRACE", "1");

    let mut app = Tracky::new();
    app.state.handle_event(model::Event::SetNoteField {
        note: NoteName::A,
        octave_modifier: 0,
    });

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.init()?;

    let (event_tx, event_rx) = channel();

    log::setup(event_tx.clone())?;

    let input_thread_event_tx = event_tx.clone();
    thread::Builder::new()
        .name("input thread".into())
        .spawn(move || 'input_loop: loop {
            macro_rules! send {
                ($e:expr) => {
                    if let Err(e) = input_thread_event_tx.send($e) {
                        ::log::warn!("Event channel is closed, input thread is done: {e}");
                        break 'input_loop;
                    }
                };
            }
            match ratatui::crossterm::event::read() {
                Ok(ratatui::crossterm::event::Event::Key(key_event))
                    if key_event.kind == KeyEventKind::Press =>
                {
                    send!(Event::Key(key_event));
                }
                Ok(ratatui::crossterm::event::Event::Resize(w, h)) => {
                    send!(Event::Resize {
                        width: w,
                        height: h,
                    });
                }
                Err(err) => send!(Event::Panic(err.into())),
                _ => {}
            }
        })
        .expect("Could not create input thread");

    macro_rules! send {
        ($e:expr) => {
            event_tx.send($e).unwrap()
        };
    }

    while app.running {
        tui.draw(&mut app)?;
        let mut event = event_rx.recv()?;
        // debug!("{event:?}");
        if let Some(popup) = app.popup_state.last_mut() {
            if let Some(unprocessed_event) = popup.handle_event(event, event_tx.clone()) {
                event = unprocessed_event;
            } else {
                continue;
            }
        }

        match event {
            Event::Key(key_event) => {
                if let Some(event) = app.keybindings.action(key_event.code, app.input_context()) {
                    send!(event);
                }
            }
            Event::Action(action) => match action {
                Action::TogglePlay => {
                    if app.state.is_playing() {
                        send!(Event::State(model::Event::StopSongPlayback));
                    } else if let Some(audio_state) = app.audio_state.as_ref() {
                        send!(Event::State(model::Event::StartSongPlayback {
                            frame_rate: audio_state.player.frame_rate,
                        }));
                    } else {
                        warn!("Select a device with F1 to play the song")
                    }
                }
                Action::WriteLogsOnDisk => {
                    if let Err(e) = write_logs_to_file("tracky.log") {
                        error!("Could not write log: {e}");
                    }
                }
                Action::ClearLogsPanel => crate::log::clear_entries(),
                Action::ToggleLogsPanel => app.display_log_console = !app.display_log_console,
                Action::Cancel => send!(Event::ExitApp),
                Action::Confirm => {}
                Action::RequestOpenDeviceSelectionPopup => {
                    send!(Event::StartLoading);
                    let event_tx_clone = event_tx.clone();
                    thread::spawn(move || {
                        event_tx_clone
                            .send(Event::LoadingDone(AsyncAction::OpenDeviceSelectionPopup(
                                Hosts::load(),
                            )))
                            .unwrap();
                    });
                }
                Action::Move(direction) => send!(Event::State(model::Event::MoveCursor(direction))),
                Action::Forward => todo!(),
                Action::Backward => todo!(),
            },
            Event::Panic(error) => {
                panic!("{error:?}");
            }
            Event::Composite(events) => {
                for event in events {
                    send!(event);
                }
            }
            Event::Resize { width, height } => info!("{width}x{height}"),
            Event::AsyncAction(async_action) => match async_action {
                event::AsyncAction::OpenDeviceSelectionPopup(hosts) => {
                    app.popup_state
                        .push(Popup::AudioDeviceSelection(hosts.into()));
                }
            },
            Event::StartLoading => app.loader_count += 1,
            Event::LoadingDone(async_action) => {
                app.loader_count = app.loader_count.saturating_sub(1);
                send!(Event::AsyncAction(async_action));
            }
            Event::ClosePopup => app.close_popup(),
            Event::SetPlayingDevice(device) => app.selected_output_device = Some(device),
            Event::State(event) => {
                app.state.handle_event(event.clone());
                app.send_player_state_event(event);
            }
            Event::AudioCallback(event) => app.state.handle_event(event),
            Event::ExitApp => app.exit(),
            Event::StartAudioPlayer => app.start_audio_player(event_tx.clone()),
            Event::RequestRedraw => {}
            Event::StopAudioPlayer(error) => {
                if let Some(err) = error {
                    error!("Audio player stopped: {err}");
                } else {
                    info!("Audio played stopped");
                }
                app.stop_audio_player();
                app.state.handle_event(model::Event::StopSongPlayback);
            }
            Event::Text(_) => unreachable!(), // For now
            Event::TextSubmitted(id, value) => {
                info!("Text submitted: {value} with id: {id}");
            } // For now
        }
    }
    tui.exit()?;

    Ok(())
}
