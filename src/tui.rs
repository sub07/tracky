use crate::tracky::Tracky;
use crate::view::render_root;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{crossterm, Terminal};
use std::io;
use std::panic;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
}

/// This replaces the standard color_eyre panic and error hooks with hooks that
/// restore the terminal before printing the panic or error.
fn install_hooks<B: Backend>() -> color_eyre::Result<()> {
    // add any extra configuration you need to the hook builder
    let hook_builder = color_eyre::config::HookBuilder::default();
    let (panic_hook, eyre_hook) = hook_builder.into_hooks();

    // convert from a color_eyre PanicHook to a standard panic hook
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        Tui::<B>::reset().unwrap();
        panic_hook(panic_info);
    }));

    // convert from a color_eyre EyreHook to a eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        Tui::<B>::reset().unwrap();
        eyre_hook(error)
    }))?;

    Ok(())
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> eyre::Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        install_hooks::<B>()?;

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: ratatui::Terminal::draw
    /// [`rendering`]: crate::ui::render
    pub fn draw(&mut self, app: &mut Tracky) -> eyre::Result<()> {
        self.terminal.draw(|frame| render_root(app, frame))?;
        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> eyre::Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> eyre::Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
