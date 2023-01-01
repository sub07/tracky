use crate::renderer::Renderer;
use crate::theme::Theme;

pub mod pattern_line;
pub mod pattern;
pub mod patterns;
pub mod field;

pub trait Draw {
    type DrawData = ();
    fn draw(&self, renderer: &mut Renderer, x: i32, y: i32, theme: &Theme, data: Self::DrawData);
}
