use crate::{
    event::{self, Action, Event, EventAware},
    keybindings::InputContext,
    utils::Direction,
    view::{centered_line, margin, responsive_centered_rect, theme::THEME, widget::slider::Slider},
    EventSender,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Clear, Gauge, Paragraph, Widget},
};
use tui_input::{Input, InputRequest};

pub struct Popup {
    label: String,
    value: i32,
    min: i32,
    max: i32,
    step: i32,
    on_submit: Box<dyn Fn(i32, EventSender)>,
}

impl Popup {
    pub fn new<OnSubmitFn>(
        label: String,
        initial_value: i32,
        min: i32,
        max: i32,
        step: i32,
        on_submit: OnSubmitFn,
    ) -> Popup
    where
        OnSubmitFn: Fn(i32, EventSender) + 'static,
    {
        Self {
            label,
            value: initial_value,
            min,
            max,
            step,
            on_submit: Box::new(on_submit),
        }
    }

    fn increment(&mut self) {
        self.value = self
            .value
            .saturating_add(self.step)
            .clamp(self.min, self.max);
    }

    fn decrement(&mut self) {
        self.value = self
            .value
            .saturating_sub(self.step)
            .clamp(self.min, self.max);
    }
}

pub enum PopupEvent {
    Close,
    Submit,
    Increment,
    Decrement,
    Input(event::Text),
}

impl EventAware<PopupEvent> for Popup {
    fn map_event(&self, event: &crate::event::Event) -> Option<PopupEvent> {
        match event {
            Event::Action(Action::Cancel) => Some(PopupEvent::Close),
            Event::Action(Action::Confirm) => Some(PopupEvent::Submit),
            Event::Action(Action::Move(direction)) => match direction {
                Direction::Left => Some(PopupEvent::Decrement),
                Direction::Right => Some(PopupEvent::Increment),
                _ => None,
            },
            Event::Text(text_event) => None,
            _ => None,
        }
    }

    fn update(&mut self, event: PopupEvent, event_sender: EventSender) {
        match event {
            PopupEvent::Close => event_sender.send_event(Event::ClosePopup).unwrap(),
            PopupEvent::Input(text_event) => {}
            PopupEvent::Submit => (self.on_submit)(self.value, event_sender),
            PopupEvent::Increment => self.increment(),
            PopupEvent::Decrement => self.decrement(),
        }
    }

    fn input_context(&self) -> crate::keybindings::InputContext {
        InputContext::Global
    }
}

impl Popup {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let area = responsive_centered_rect(
            area,
            Constraint::Percentage(30),
            Constraint::Length(30),
            Constraint::Length(50),
            Constraint::Length(3),
        );

        let block = Block::bordered().title("Global volume");

        let area = {
            let inner = block.inner(area);
            Clear.render(area, buf);
            block.render(area, buf);
            inner
        };

        let slider_area = centered_line(area);
        Slider::new(self.min, self.max, self.value).render(slider_area, buf);
    }
}
