use iced::{
    font::{self, Stretch, Weight},
    Font,
};

pub mod component;
pub mod widget;

const MONOSPACED_FONT: Font = Font {
    family: font::Family::Name("Roboto Mono"),
    weight: Weight::Light,
    stretch: Stretch::Normal,
    style: font::Style::Normal,
};

pub trait CustomRenderer:
    iced::advanced::text::Renderer<Font = iced::Font, Paragraph = iced_graphics::text::Paragraph>
{
}

impl CustomRenderer for iced::Renderer {}
