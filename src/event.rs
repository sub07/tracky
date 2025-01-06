pub enum Event {
    Key(ratatui::crossterm::event::KeyEvent),
    App(crate::keybindings::Action),
    Panic(anyhow::Error),
}