use iced::{Font, Theme};

pub mod component;
pub mod widget;

pub trait CustomRenderer: iced::advanced::text::Renderer<Theme = Theme, Font = Font> {}
impl CustomRenderer for iced::Renderer {}
