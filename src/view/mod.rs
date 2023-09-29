use iced::{Font, Theme, font::{self, Stretch, Weight}};

pub mod component;
pub mod widget;

const MONOSPACED_FONT: Font = Font {
    family: font::Family::Name("Roboto Mono"),
    monospaced: true,
    stretch: Stretch::Normal,
    weight: Weight::Light,
};

pub trait CustomRenderer: iced::advanced::text::Renderer<Theme = Theme, Font = Font> {}
impl CustomRenderer for iced::Renderer {}
