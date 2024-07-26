#![feature(const_fn_floating_point_arithmetic)]
#![feature(array_chunks)]
#![feature(vec_into_raw_parts)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]

use std::time::Instant;
use std::{env, io};

use ::log::info;
use handler::handle_key_events;
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
mod utils;
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

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.init()?;

    let mut last_tick_time = Instant::now();

    let mut get_delta = || {
        let delta = last_tick_time.elapsed();
        last_tick_time = Instant::now();
        delta
    };

    while app.running {
        tui.draw(&mut app)?;
        if let Some(timeout) = app.poll_event_timeout
            && !event::poll(timeout)?
        {
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
