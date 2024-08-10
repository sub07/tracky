#![feature(const_fn_floating_point_arithmetic)]
#![feature(array_chunks)]
#![feature(vec_into_raw_parts)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]

use std::time::{Duration, Instant};
use std::{env, io};

use ::log::info;
use handler::handle_key_events;
use model::pattern::{Field, HexDigit, NoteFieldValue, NoteName, OctaveValue, PatternLine};
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::Terminal;
use tracky::Tracky;
use tui::Tui;

mod audio;
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

const EVENT_POLL_TIMEOUT: Duration = Duration::from_millis(22);

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

    let mut last_time = Instant::now();

    let mut get_delta = || {
        let delta = last_time.elapsed();
        last_time = Instant::now();
        delta
    };

    while app.running {
        tui.draw(&mut app)?;
        if !event::poll(EVENT_POLL_TIMEOUT)? {
            app.tick(get_delta());
            continue;
        }
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                handle_key_events(key_event, &mut app)?;
            }
            event::Event::Resize(w, h) => info!("{w}x{h}"),
            _ => {}
        }
        app.tick(get_delta());
    }

    tui.exit()?;
    Ok(())
}
