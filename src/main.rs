use iced::event::Event;
use iced::keyboard::KeyCode;
use iced::{
    executor, subscription, Application, Command, Element, Renderer, Settings, Subscription, Theme,
};

use iced_native::widget::scrollable::Properties;
use model::pattern::Pattern;
use rust_utils_macro::New;

use view::component::pattern::pattern_component;

use crate::model::pattern::{Column, ColumnLineElement};

mod model;
mod view;

pub fn main() -> iced::Result {
    Tracky::run(Settings {
        default_font: Some(include_bytes!("../font.ttf")),
        ..Default::default()
    })
}

#[derive(Default, New)]
struct Tracky {
    pattern: Pattern,
    cursor_x: i32,
    cursor_y: i32,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
}

impl Application for Tracky {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let model = Tracky {
            pattern: Pattern {
                columns: vec![Column::default(); 10],
            },
            ..Default::default()
        };
        (model, Command::none())
    }

    fn title(&self) -> String {
        "Tracky".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        if let Message::EventOccurred(Event::Keyboard(iced::keyboard::Event::KeyPressed {
            key_code,
            ..
        })) = message
        {
            match key_code {
                KeyCode::Left => self.cursor_x -= 1,
                KeyCode::Right => self.cursor_x += 1,
                KeyCode::Up => self.cursor_y -= 1,
                KeyCode::Down => self.cursor_y += 1,
                _ => {}
            }
            self.cursor_x = i32::rem_euclid(
                self.cursor_x,
                ColumnLineElement::LINE_LEN * self.pattern.columns.len() as i32,
            );
            self.cursor_y =
                i32::rem_euclid(self.cursor_y, self.pattern.columns[0].lines.len() as i32);
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        iced::widget::scrollable(pattern_component(
            &self.pattern,
            self.cursor_x,
            self.cursor_y,
        ))
        .horizontal_scroll(Properties::default())
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::EventOccurred)
    }
}
