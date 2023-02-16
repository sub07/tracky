use std::any::Any;
use rust_utils_macro::New;
use sdl2::keyboard::Keycode::V;
use crate::renderer::WindowRenderer;
use crate::theme::Theme;
use crate::Vec2;

pub mod column_line;
pub mod column;
pub mod pattern;
pub mod field;
pub mod patterns;

pub trait Draw {
    type DrawData = ();
    fn draw<Renderer: WindowRenderer>(&self, renderer: &mut Renderer, x: i32, y: i32, theme: &Theme, data: Self::DrawData);
}

#[derive(New, Default)]
pub struct ViewInner<ViewState> {
    position: Option<Vec2>,
    size: Option<Vec2>,
    state: Option<ViewState>,
    children: Vec<Box<dyn Any>>,
}

pub trait View : Bound {
    fn init<R: WindowRenderer>(&mut self, renderer: &R);
    fn draw<R: WindowRenderer>(&self, renderer: &mut R, theme: &Theme);
}

pub trait Bound {
    fn size(&self) -> Vec2;
}
