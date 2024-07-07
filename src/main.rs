#![feature(const_fn_floating_point_arithmetic)]
#![feature(array_chunks)]
#![feature(vec_into_raw_parts)]

use std::time::Duration;
use std::{env, io};

use event::{Event, EventHandler};
use log::DisplayLogExt;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tracky::Tracky;
use tui::Tui;

mod audio;
mod event;
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

#[tokio::main]
async fn main() -> eyre::Result<()> {
    #[cfg(debug_assertions)]
    env::set_var("RUST_BACKTRACE", "0");

    // Create an application.
    let mut app = Tracky::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(Duration::from_secs(100));
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handler::handle_key_events(key_event, &mut app).await?,
            // Event::Mouse(_) => {}
            Event::Resize(w, h) => {
                format!("{w}x{h}").info("terminal size");
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
