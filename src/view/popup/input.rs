use super::HandleEvent;
use crate::{
    event::{self, Action, Event},
    keybindings::InputContext,
    view::{centered_line, centered_rect, margin, responsive_centered_rect},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};
use std::{marker::PhantomData, sync::mpsc::Sender};
use tui_input::{backend::crossterm::EventHandler, Input, InputRequest};
use uid::{Id, IdU64};

pub type InputId = Id<()>;

pub struct Popup {
    id: InputId,
    label: String,
    input: Input,
    input_validator: Box<dyn Fn(char) -> bool>,
    submit_validator: Box<dyn Fn(&str) -> bool>,
}

impl Popup {
    pub fn new<F, F2>(
        id: InputId,
        label: String,
        initial_value: Option<String>,
        input_validator: F,
        submit_validator: F2,
    ) -> Popup
    where
        F: Fn(char) -> bool + 'static,
        F2: Fn(&str) -> bool + 'static,
    {
        Popup {
            id,
            label,
            input: initial_value.unwrap_or_default().into(),
            input_validator: Box::new(input_validator),
            submit_validator: Box::new(submit_validator),
        }
    }
}

pub enum PopupEvent {
    Close,
    Submit,
    Input(event::Text),
}

impl HandleEvent<PopupEvent> for Popup {
    fn map_event(&self, event: &crate::event::Event) -> Option<PopupEvent> {
        match event {
            Event::Action(Action::Cancel) => Some(PopupEvent::Close),
            Event::Action(Action::Confirm) => Some(PopupEvent::Submit),
            Event::Text(text_event) => Some(PopupEvent::Input(text_event.clone())),
            _ => None,
        }
    }

    fn update(&mut self, event: PopupEvent, event_tx: Sender<crate::event::Event>) {
        match event {
            PopupEvent::Close => event_tx.send(Event::ClosePopup).unwrap(),
            PopupEvent::Input(text_event) => {
                let input_request = match text_event {
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
            PopupEvent::Submit => {
                if (self.submit_validator)(self.input.value()) {
                    event_tx
                        .send(Event::Composite(vec![
                            Event::ClosePopup,
                            Event::TextSubmitted(self.id, self.input.value().to_owned()),
                        ]))
                        .unwrap()
                }
            }
        }
    }

    fn input_context(&self) -> crate::keybindings::InputContext {
        InputContext::Text
    }
}

impl Popup {
    const INPUT_BG_COLOR: Color = Color::Gray;
    const INPUT_FG_COLOR: Color = Color::Black;
    const INPUT_CURSOR_BG_COLOR: Color = Color::DarkGray;
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let area = responsive_centered_rect(
            area,
            Constraint::Percentage(30),
            Constraint::Min(40),
            Constraint::Max(60),
            Constraint::Percentage(30),
        );

        let block = Block::bordered();

        let area = {
            let inner = block.inner(area);
            Clear.render(area, buf);
            block.render(area, buf);
            inner
        };

        let area = margin(area, 2);

        let [label_area, input_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
                .spacing(1)
                .areas(area);

        Line::from(self.label.as_str()).render(label_area, buf);
        Paragraph::new(self.input.value())
            .bg(Self::INPUT_BG_COLOR)
            .fg(Self::INPUT_FG_COLOR)
            .render(input_area, buf);

        if let Some(cell) = buf.cell_mut((input_area.x + self.input.cursor() as u16, input_area.y))
        {
            cell.set_bg(Self::INPUT_CURSOR_BG_COLOR);
        }
    }
}
