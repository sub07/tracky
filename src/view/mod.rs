use std::any::Any;

use crate::renderer::Renderer;
use crate::theme::Theme;

pub mod column_line;
pub mod column;
pub mod pattern;
pub mod field;
pub mod patterns;

pub trait Draw {
    type DrawData = ();
    fn draw<R: Renderer>(&self, renderer: &mut R, x: i32, y: i32, theme: &Theme, data: Self::DrawData);
}
