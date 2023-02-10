use crate::renderer::{WindowRenderer};
use crate::theme::Theme;

pub mod column_line;
pub mod column;
pub mod pattern;
pub mod field;
pub mod patterns;

pub trait Draw {
    type DrawData = ();
    fn draw<Renderer: WindowRenderer>(&self, renderer: &mut Renderer, x: i32, y: i32, theme: &Theme, data: Self::DrawData);
}

pub trait ViewDraw {
    fn draw<Renderer: WindowRenderer>(&self, renderer: &mut Renderer, theme: &Theme);
}
