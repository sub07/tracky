use crate::renderer::Renderer;

pub mod pattern_line;
pub mod pattern;
pub mod patterns;
pub mod field;

pub trait Draw {
    type DrawData = ();
    fn draw(&self, renderer: &mut Renderer, x: i32, y: i32, data: Self::DrawData);
}
