use crate::{
    audio::Decibels,
    event::{self, Action, Event, HandleAction},
    keybindings::InputContext,
    utils::Direction,
    view::{
        centered_line, render_block_and_get_inner, responsive_centered_rect, widget::slider::Slider,
    },
    EventSender,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::ToLine,
    widgets::{Block, Widget},
};

const DECIBELS_STEP: f32 = 0.1;

pub struct Popup {
    title: &'static str,
    value: Decibels,
    on_submit: Box<dyn Fn(Decibels, EventSender)>,
}

impl Popup {
    pub fn new<OnSubmitFn>(
        title: &'static str,
        initial_value: Decibels,
        on_submit: OnSubmitFn,
    ) -> Popup
    where
        OnSubmitFn: Fn(Decibels, EventSender) + 'static,
    {
        Self {
            value: initial_value,
            on_submit: Box::new(on_submit),
            title,
        }
    }

    fn increment(&mut self) {
        self.value = self.value + DECIBELS_STEP;
    }

    fn decrement(&mut self) {
        self.value = self.value - DECIBELS_STEP;
    }
}

pub enum PopupAction {
    Close,
    Submit,
    Increment,
    Decrement,
    Input(event::Text), // TODO: accept input to set value
}

impl HandleAction<PopupAction> for Popup {
    fn map_action(&self, event: &Action) -> Option<PopupAction> {
        match event {
            Action::Cancel => Some(PopupAction::Close),
            Action::Confirm => Some(PopupAction::Submit),
            Action::Move(direction) => match direction {
                Direction::Left => Some(PopupAction::Decrement),
                Direction::Right => Some(PopupAction::Increment),
                _ => None,
            },
            Action::Text(text_event) => None,
            _ => None,
        }
    }

    fn update(&mut self, event: PopupAction, event_sender: EventSender) {
        match event {
            PopupAction::Close => event_sender.send_event(Event::ClosePopup).unwrap(),
            PopupAction::Input(text_event) => {}
            PopupAction::Submit => (self.on_submit)(self.value, event_sender),
            PopupAction::Increment => self.increment(),
            PopupAction::Decrement => self.decrement(),
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

        let area = render_block_and_get_inner(Block::bordered().title(self.title), area, buf);

        let area = centered_line(area);
        let [slider_area, text_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(7)]).areas(area);
        Slider::new(Decibels::MIN_VALUE, Decibels::MAX_VALUE, self.value.value())
            .render(slider_area, buf);

        let value_text = format!("{:.1}dB", self.value.value());
        value_text.to_line().right_aligned().render(text_area, buf);
    }
}
