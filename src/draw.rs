use crate::renderer::Renderer;

pub trait Draw {
    type DrawData = ();
    fn draw(&self, renderer: &mut Renderer, x: i32, y: i32, data: Self::DrawData);
}