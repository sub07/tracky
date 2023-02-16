use std::path::Path;

use rust_utils_macro::New;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::mono_font_atlas::{MonoFontAtlas, TextStyle};
use crate::Vec2;

pub trait WindowRenderer {
    fn clear(&mut self, color: (u8, u8, u8));
    fn set_clip<V: Into<Vec2>, V1: Into<Vec2>>(&mut self, pos: V, size: V1);
    fn draw_text<S: AsRef<str>, V: Into<Vec2>>(&mut self, text: S, pos: V, text_style: TextStyle);
    fn draw_rect<V: Into<Vec2>, V1: Into<Vec2>>(&mut self, pos: V, size: V1, color: (u8, u8, u8), filled: bool);
    fn present(&mut self);
    fn glyph_size(&self) -> Vec2;
    fn window_size(&self) -> Vec2;
    #[deprecated]
    fn glyph_width(&self) -> i32;
    #[deprecated]
    fn glyph_height(&self) -> i32;
    fn set_size<V: Into<Vec2>>(&mut self, size: V);
    fn set_window_title<S: AsRef<str>>(&mut self, title: S);
    fn set_draw_origin<V: Into<Vec2>>(&mut self, new_origin: V);
}

pub struct SdlRenderer<'a> {
    canvas: WindowCanvas,
    font: MonoFontAtlas<'a>,
}

impl<'a> SdlRenderer<'a> {
    pub fn new<P: AsRef<Path>>(canvas: WindowCanvas, texture_creator: &'a TextureCreator<WindowContext>, default_font_path: P, default_font_size: u16, default_font_glyphs: &'static str) -> SdlRenderer<'a> {
        let font = MonoFontAtlas::new(texture_creator, default_font_path, default_font_size, default_font_glyphs);
        SdlRenderer { canvas, font }
    }
}

fn rect_from_pos_size<V: Into<Vec2>, V1: Into<Vec2>>(pos: V, size: V1) -> Rect {
    let ([x, y], [w, h]) = (*pos.into().as_slice(), *size.into().as_slice());
    Rect::new(x, y, w as u32, h as u32)
}

impl WindowRenderer for SdlRenderer<'_> {
    fn clear(&mut self, color: (u8, u8, u8)) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn set_clip<V: Into<Vec2>, V1: Into<Vec2>>(&mut self, pos: V, size: V1) {
        self.canvas.set_clip_rect(rect_from_pos_size(pos, size));
    }

    fn draw_text<S: AsRef<str>, V: Into<Vec2>>(&mut self, text: S, pos: V, text_style: TextStyle) {
        let [x, y] = *pos.into().as_slice();
        self.font.draw(&mut self.canvas, text, x, y, text_style);
    }

    fn draw_rect<V: Into<Vec2>, V1: Into<Vec2>>(&mut self, pos: V, size: V1, color: (u8, u8, u8), filled: bool) {
        self.canvas.set_draw_color(color);
        let rect = rect_from_pos_size(pos, size);
        if filled {
            self.canvas.fill_rect(rect).unwrap()
        } else {
            self.canvas.draw_rect(rect).unwrap()
        }
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn glyph_size(&self) -> Vec2 {
        [self.font.glyph_width(), self.font.glyph_height()].into()
    }

    fn window_size(&self) -> Vec2 {
        let (w, h) = self.canvas.output_size().unwrap();
        [w as i32, h as i32].into()
    }

    fn glyph_width(&self) -> i32 {
        self.glyph_size()[0]
    }

    fn glyph_height(&self) -> i32 {
        self.glyph_size()[1]
    }

    fn set_size<V: Into<Vec2>>(&mut self, size: V) {
        let [width, height] = *size.into().as_slice();
        self.canvas.window_mut().set_size(width as u32, height as u32).unwrap()
    }

    fn set_window_title<S: AsRef<str>>(&mut self, title: S) {
        self.canvas.window_mut().set_title(title.as_ref()).unwrap();
    }

    fn set_draw_origin<V: Into<Vec2>>(&mut self, _: V) { panic!("Should not be called") }
}

#[derive(New)]
pub struct RendererProxy<'a, Renderer> {
    renderer: &'a mut Renderer,
    #[new_default]
    origin: Vec2,
    #[new_default]
    pen: Vec2,
}

impl<Renderer: WindowRenderer> WindowRenderer for RendererProxy<'_, Renderer> {
    fn clear(&mut self, color: (u8, u8, u8)) {
        self.renderer.clear(color);
    }

    fn set_clip<V: Into<Vec2>, V1: Into<Vec2>>(&mut self, pos: V, size: V1) {
        self.renderer.set_clip(pos, size);
    }

    fn draw_text<S: AsRef<str>, V: Into<Vec2>>(&mut self, text: S, pos: V, text_style: TextStyle) {
        self.renderer.draw_text(text, self.pen + pos.into() + self.origin, text_style);
        // self.advance_pen();
    }

    fn draw_rect<V: Into<Vec2>, V1: Into<Vec2>>(&mut self, pos: V, size: V1, color: (u8, u8, u8), filled: bool) {
        self.renderer.draw_rect(pos.into() + self.origin, size, color, filled);
    }

    fn present(&mut self) {
        self.renderer.present();
    }

    fn glyph_size(&self) -> Vec2 {
        self.renderer.glyph_size()
    }

    fn window_size(&self) -> Vec2 {
        self.renderer.window_size()
    }

    fn glyph_width(&self) -> i32 {
        self.renderer.glyph_width()
    }

    fn glyph_height(&self) -> i32 {
        self.renderer.glyph_height()
    }

    fn set_size<V: Into<Vec2>>(&mut self, size: V) {
        self.renderer.set_size(size);
    }

    fn set_window_title<S: AsRef<str>>(&mut self, title: S) {
        self.renderer.set_window_title(title);
    }

    fn set_draw_origin<V: Into<Vec2>>(&mut self, new_origin: V) {
        self.origin = new_origin.into();
    }
}
