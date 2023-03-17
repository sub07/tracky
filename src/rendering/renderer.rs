use rust_utils_macro::New;
use softbuffer::Surface;
use tiny_skia::{Color, Paint, Pixmap};
use tiny_skia_path::{PathBuilder, Rect, Stroke, Transform};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::{Scalar, Vec2};
use crate::rendering::color::IntoColor;
use crate::rendering::font::{Font, TextAlignment, TextStyle};

trait PaintBuilder<'a> {
    fn from_solid_color(color: Color) -> Paint<'a> {
        let mut paint = Paint::default();
        paint.set_color(color);
        paint
    }
}

impl<'a> PaintBuilder<'a> for Paint<'a> {}

pub trait Renderer {
    fn clear(&mut self, color: (u8, u8, u8));
    fn draw_text(&mut self, text: &str, position: Vec2, text_style: TextStyle);
    fn draw_rect(&mut self, position: Vec2, size: Vec2, color: (u8, u8, u8), filled: bool);
    fn present(&mut self);
    fn window_size(&self) -> Vec2;
    fn glyph_size(&self) -> Vec2;
    fn set_window_size(&mut self, size: Vec2);
    fn set_window_title(&mut self, title: &str);
}

#[derive(New)]
pub struct SkiaRenderer {
    pub(super) window: Window,
    pub(super) window_surface: Surface,
    pub(super) screen: Pixmap,
    font: Font,
}

impl Renderer for SkiaRenderer {
    fn clear(&mut self, color: (u8, u8, u8)) {
        self.screen.fill(color.into_color());
    }

    fn draw_text(&mut self, text: &str, mut position: Vec2, text_style: TextStyle) {
        let text_width = text.len() as i32 * self.glyph_size().w();
        if text_style.alignment == TextAlignment::Right {
            position[0] -= text_width;
        }
        if let Some(bg_color) = text_style.background_color {
            self.draw_rect(position, [text_width, self.glyph_size().h()].into(), bg_color, true);
        }
        self.font.draw_text_on(&mut self.screen, text, position, text_style);
    }

    fn draw_rect(&mut self, position: Vec2, size: Vec2, color: (u8, u8, u8), filled: bool) {
        let [x, y] = *position.as_slice();
        let [w, h] = *size.as_slice();
        let rect = Rect::from_xywh(x as f32, y as f32, w as f32, h as f32).unwrap();
        let paint = Paint::from_solid_color(color.into_color());
        if filled {
            self.screen.fill_rect(rect, &paint, Transform::identity(), None);
        } else {
            let rect_path = PathBuilder::from_rect(rect);
            self.screen.stroke_path(&rect_path, &paint, &Stroke::default(), Transform::identity(), None);
        }
    }

    fn present(&mut self) {
        fn u8_to_u32(arr: &[u8]) -> Option<&[u32]> {
            if arr.len() % 4 != 0 { return None; }
            let len = arr.len() / 4;
            let ptr = arr.as_ptr() as *const u32;
            unsafe {
                Some(std::slice::from_raw_parts(ptr, len))
            }
        }

        let mut window_buffer = self.window_surface.buffer_mut().unwrap();
        for pixel in window_buffer.iter_mut() {
            *pixel = 0x00FF0000;
        }
        window_buffer.present().unwrap();

        // self.window_surface.set_buffer(
        //     u8_to_u32(self.screen.data()).unwrap(),
        //     self.screen.width() as u16,
        //     self.screen.height() as u16,
        // );
    }

    fn window_size(&self) -> Vec2 {
        let size = self.window.inner_size();
        Vec2::new(size.width as Scalar, size.height as Scalar)
    }

    fn glyph_size(&self) -> Vec2 {
        self.font.glyph_size
    }

    fn set_window_size(&mut self, size: Vec2) {
        let [w, h] = *size.as_slice();
        self.window.set_inner_size(PhysicalSize::new(w, h));
        self.window_surface.resize(w as u32, h as u32).unwrap();
    }

    fn set_window_title(&mut self, title: &str) {
        self.window.set_title(title);
    }
}
