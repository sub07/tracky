use crate::log::write_logs_to_file;
use crate::tracky::Tracky;
use crate::view::render_root;
use ratatui::backend::Backend;
use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{crossterm, Terminal};
use std::io;
use std::panic;

#[derive(Debug)]
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
}

fn install_hooks<B: Backend>() -> anyhow::Result<()> {
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        Tui::<B>::reset().unwrap();
        let _ = write_logs_to_file("tracky.log");
        panic_hook(panic_info);
    }));

    Ok(())
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen)?;

        install_hooks::<B>()?;

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut Tracky) -> anyhow::Result<()> {
        self.terminal.draw(|frame| render_root(app, frame))?;
        Ok(())
    }

    fn reset() -> anyhow::Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn exit(&mut self) -> anyhow::Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
