use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    style::Stylize,
    widgets::{Paragraph, Widget, WidgetRef},
};
use tui_input::InputRequest;

use crate::{event, view::theme::THEME};

pub struct State {
    label: String,
    input: tui_input::Input,
    input_validator: Box<dyn Fn(char) -> bool>,
}

impl State {
    pub fn new<F>(label: String, initial_value: Option<String>, input_validator: F) -> State
    where
        F: Fn(char) -> bool + 'static,
    {
        Self {
            label,
            input: initial_value.unwrap_or_default().into(),
            input_validator: Box::new(input_validator),
        }
    }

    pub fn handle(&mut self, event: event::Text) {
        let input_request = match event {
            event::Text::WriteDataAtCursor(c) => {
                (self.input_validator)(c).then_some(InputRequest::InsertChar(c))
            }
            event::Text::RemoveCharAtCursor => Some(InputRequest::DeletePrevChar),
            event::Text::MoveCursorLeft => Some(InputRequest::GoToPrevChar),
            event::Text::MoveCursorRight => Some(InputRequest::GoToNextChar),
        };
        if let Some(req) = input_request {
            self.input.handle(req);
        }
    }
}

impl WidgetRef for State {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let [label_area, input_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);
        self.label.render_ref(label_area, buf);
        let input_scroll = self.input.visual_scroll(input_area.width as usize - 1);
        Paragraph::new(&self.input.value()[input_scroll..])
            .bg(THEME.elevated_background_2)
            .fg(THEME.on_elevated_background_2)
            .render(input_area, buf);
    }
}
