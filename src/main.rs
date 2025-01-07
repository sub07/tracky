use std::sync::mpsc::channel;
use std::{env, io, thread};

use ::log::{error, info};
use event::Event;
use model::pattern::{Field, HexDigit, NoteFieldValue, NoteName, OctaveValue, PatternLine};
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::Terminal;
use tracky::Tracky;
use tui::Tui;
use view::popup::Popup;

mod audio;
mod event;
mod handler;
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

    *app.patterns.current_line_mut() = PatternLine {
        note: Field::new(NoteFieldValue::Note(NoteName::C, OctaveValue::OCTAVE_5)),
        velocity: Field::new((HexDigit::HEX_1, HexDigit::HEX_0)),
        instrument: Field::new((HexDigit::HEX_0, HexDigit::HEX_0)),
    };

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
                if let Some(action) = app.keybindings.action(key_event.code, app.input_context()) {
                    event_tx.send(Event::Action(action)).unwrap();
                }
            }
            Event::Action(action) => {
                if let Err(err) = handler::handle_action(action, &mut app, event_tx.clone()) {
                    error!("{err}");
                }
            }
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
        }
    }

    tui.exit()?;

    // let (state_event_tx, state_event_rx) = std::sync::mpsc::channel();
    // let (playback_event_tx, playback_event_rx) = std::sync::mpsc::channel();

    // struct Sine {
    //     phase: f32,
    //     freq: f32,
    // }

    // let player = AudioPlayerBuilder::new()
    //     .name("Test player")
    //     .initial_state(Sine {
    //         freq: 440.0,
    //         phase: 0.0,
    //     })
    //     .state_event_rx(state_event_rx)
    //     .playback_event_tx(playback_event_tx)
    //     .processor(
    //         |out: &mut [f32],
    //          frame_rate: f32,
    //          state: &mut Sine,
    //          events: &[String],
    //          playback_event_tx: std::sync::mpsc::Sender<String>| {
    //             for event in events {
    //                 match event.as_str() {
    //                     "up" => state.freq += 20.0,
    //                     "down" => state.freq -= 20.0,
    //                     "done" => playback_event_tx.send("done".to_string()).unwrap(),
    //                     _ => {}
    //                 }
    //             }

    //             let period = 1.0 / frame_rate;

    //             for out in out.chunks_exact_mut(2) {
    //                 state.phase += 2.0 * PI * state.freq * period;
    //                 let s = state.phase.sin() * 0.3;
    //                 out[0] = s;
    //                 out[1] = s;
    //             }
    //         },
    //     )
    //     .build()
    //     .into_player()
    //     .unwrap();

    // std::thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(1));
    //     state_event_tx.send("up".to_string()).unwrap();
    //     thread::sleep(Duration::from_secs_f32(0.5));
    //     state_event_tx.send("up".to_string()).unwrap();
    //     thread::sleep(Duration::from_secs_f32(0.5));
    //     state_event_tx.send("up".to_string()).unwrap();
    //     thread::sleep(Duration::from_secs(1));
    //     state_event_tx.send("down".to_string()).unwrap();
    //     thread::sleep(Duration::from_secs_f32(0.5));
    //     state_event_tx.send("down".to_string()).unwrap();
    //     thread::sleep(Duration::from_secs_f32(0.5));
    //     state_event_tx.send("down".to_string()).unwrap();
    //     thread::sleep(Duration::from_secs(1));
    //     state_event_tx.send("done".to_string()).unwrap();
    // });

    // loop {
    //     let event = playback_event_rx.recv().unwrap();
    //     if event.as_str() == "done" {
    //         break;
    //     }
    // }

    Ok(())
}
