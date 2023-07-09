use iced::event::Event;
use iced::keyboard::KeyCode;
use iced::widget::Row;
use iced::{
    executor, subscription, Application, Command, Element, Renderer, Sandbox, Settings,
    Subscription, Theme,
};
use iter_tools::Itertools;
use rust_utils_macro::New;

use crate::widget::input_unit::input_unit;

mod widget;

pub fn main() -> iced::Result {
    Tracky::run(Settings {
        default_font: Some(include_bytes!("../font.ttf")),
        ..Default::default()
    })
}

#[derive(Default, New)]
struct Tracky {
    column: i32,
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

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Tracky::new(1), Command::none())
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
                KeyCode::Left => self.column -= 1,
                KeyCode::Right => self.column += 1,
                _ => {}
            }
            self.column = self.column.clamp(1, 4);
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        dbg!(self.column);
        Row::with_children(
            (1..5)
                .map(|i| {
                    let selected = self.column == i;
                    input_unit(Some('5'), selected).into()
                })
                .collect_vec(),
        )
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::EventOccurred)
    }
}
