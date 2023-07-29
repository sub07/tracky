

use iced::event::Event;
use iced::keyboard::KeyCode;
use iced::{
    executor, subscription, Application, Command, Element, Renderer, Settings, Subscription, Theme,
};


use iced_native::widget::scrollable::{self, Properties};

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

#[derive(New)]
struct Tracky {
    pattern: Pattern,
    cursor_x: i32,
    cursor_y: i32,
    scroll_id: scrollable::Id,
}

impl Default for Tracky {
    fn default() -> Self {
        Self {
            pattern: Default::default(),
            cursor_x: Default::default(),
            cursor_y: Default::default(),
            scroll_id: scrollable::Id::unique(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    MoveCursor((i32, i32)),
}

fn convert_event_to_message(event: Event) -> Option<Message> {
    match event {
        Event::Keyboard(kb_event) => match kb_event {
            iced_native::keyboard::Event::KeyPressed {
                key_code,
                modifiers: _,
            } => {
                let mut cursor_x = 0;
                let mut cursor_y = 0;
                match key_code {
                    KeyCode::Left => cursor_x -= 1,
                    KeyCode::Right => cursor_x += 1,
                    KeyCode::Up => cursor_y -= 1,
                    KeyCode::Down => cursor_y += 1,
                    _ => {}
                }
                if cursor_x != 0 || cursor_y != 0 {
                    return Some(Message::MoveCursor((cursor_x, cursor_y)));
                }
            }
            iced_native::keyboard::Event::KeyReleased {
                key_code: _,
                modifiers: _,
            } => {}
            iced_native::keyboard::Event::CharacterReceived(_) => {}
            iced_native::keyboard::Event::ModifiersChanged(_) => {}
        },
        Event::Mouse(_) => {}
        Event::Window(_) => {}
        Event::Touch(_) => {}
    }
    None
}

impl Application for Tracky {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let model = Tracky {
            pattern: Pattern {
                columns: vec![Column::default(); 15],
            },
            ..Default::default()
        };
        (model, Command::none())
    }

    fn title(&self) -> String {
        "Tracky".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::EventOccurred(event) => {
                if let Some(message) = convert_event_to_message(event) {
                    return self.update(message);
                }
            }
            Message::MoveCursor((x, y)) => {
                self.cursor_x += x;
                self.cursor_y += y;

                self.cursor_x = i32::rem_euclid(
                    self.cursor_x,
                    ColumnLineElement::LINE_LEN * self.pattern.columns.len() as i32,
                );
                self.cursor_y =
                    i32::rem_euclid(self.cursor_y, self.pattern.columns[0].lines.len() as i32);

                let cursor_x_column_index = self.cursor_x / ColumnLineElement::LINE_LEN;

                return scrollable::snap_to(
                    self.scroll_id.clone(),
                    scrollable::RelativeOffset {
                        x: dbg!(
                            cursor_x_column_index as f32 / (self.pattern.columns.len() - 1) as f32
                        ),
                        y: dbg!(
                            self.cursor_y as f32 / (self.pattern.columns[0].lines.len() - 1) as f32
                        ),
                    },
                );
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        iced::widget::scrollable(pattern_component(
            &self.pattern,
            self.cursor_x,
            self.cursor_y,
        ))
        .id(self.scroll_id.clone())
        .horizontal_scroll(Properties::default())
        .vertical_scroll(Properties::default())
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::EventOccurred)
    }
}
