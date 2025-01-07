pub enum Event {
    Key(ratatui::crossterm::event::KeyEvent),
    App(crate::keybindings::Action),
    Panic(anyhow::Error),
    Async(AsyncEvent),
    Resize { width: u16, height: u16 },
    Composite(Vec<Event>),
}

pub enum AsyncEvent {
    LoadingDone,
}
