use std::path::Path;

use rust_utils_macro::New;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::mono_font_atlas::{MonoFontAtlas, TextStyle};
use crate::Vec2;

pub trait WindowRenderer {
    fn clear(&mut self, color: (u8, u8, u8));
    fn set_clip(&mut self, pos: Vec2, size: Vec2);
    fn draw_text<S: AsRef<str>>(&mut self, text: S, pos: Vec2, text_style: TextStyle);
    fn draw_rect(&mut self, pos: Vec2, size: Vec2, color: (u8, u8, u8));
    fn present(&mut self);
    fn glyph_width(&self) -> i32;
    fn glyph_height(&self) -> i32;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn set_size(&mut self, size: Vec2);
    fn set_window_title<S: AsRef<str>>(&mut self, title: S);
    fn set_draw_origin(&mut self, new_origin: Vec2);
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

impl WindowRenderer for SdlRenderer<'_> {
    fn clear(&mut self, color: (u8, u8, u8)) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn set_clip(&mut self, position: Vec2, size: Vec2) {
        self.canvas.set_clip_rect(Rect::new(position.x(), position.y(), size.x() as u32, size.y() as u32));
    }

    fn draw_text<S: AsRef<str>>(&mut self, text: S, pos: Vec2, text_style: TextStyle) {
        let [x, y] = *pos.as_slice();
        self.font.draw(&mut self.canvas, text, x, y, text_style);
    }

    fn draw_rect(&mut self, pos: Vec2, size: Vec2, color: (u8, u8, u8)) {
        self.canvas.set_draw_color(color);
        let [x, y] = *pos.as_slice();
        let [w, h] = *size.as_slice();
        self.canvas.fill_rect(Rect::new(x, y, w as u32, h as u32)).unwrap()
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn glyph_width(&self) -> i32 {
        self.font.glyph_width()
    }

    fn glyph_height(&self) -> i32 {
        self.font.glyph_height()
    }

    fn width(&self) -> i32 { self.canvas.output_size().unwrap().0 as i32 }

    fn height(&self) -> i32 { self.canvas.output_size().unwrap().1 as i32 }

    fn set_size(&mut self, size: Vec2) {
        let [width, height] = *size.as_slice();
        self.canvas.window_mut().set_size(width as u32, height as u32).unwrap()
    }

    fn set_window_title<S: AsRef<str>>(&mut self, title: S) {
        self.canvas.window_mut().set_title(title.as_ref()).unwrap();
    }

    fn set_draw_origin(&mut self, _: Vec2) { panic!("Should not be called") }
}

#[derive(New)]
pub struct RendererProxy<'a, Renderer> {
    sdl_renderer: &'a mut Renderer,
    #[new_default]
    origin: Vec2,
}

impl<Renderer: WindowRenderer> WindowRenderer for RendererProxy<'_, Renderer> {
    fn clear(&mut self, color: (u8, u8, u8)) {
        self.sdl_renderer.clear(color)
    }

    fn set_clip(&mut self, pos: Vec2, size: Vec2) {
        self.sdl_renderer.set_clip(pos, size)
    }

    fn draw_text<S: AsRef<str>>(&mut self, text: S, pos: Vec2, text_style: TextStyle) {
        self.sdl_renderer.draw_text(text, pos + self.origin, text_style)
    }

    fn draw_rect(&mut self, pos: Vec2, size: Vec2, color: (u8, u8, u8)) {
        self.sdl_renderer.draw_rect(pos + self.origin, size, color)
    }

    fn present(&mut self) {
        self.sdl_renderer.present()
    }

    fn glyph_width(&self) -> i32 {
        self.sdl_renderer.glyph_width()
    }

    fn glyph_height(&self) -> i32 {
        self.sdl_renderer.glyph_height()
    }

    fn width(&self) -> i32 {
        self.sdl_renderer.width()
    }

    fn height(&self) -> i32 {
        self.sdl_renderer.height()
    }

    fn set_size(&mut self, size: Vec2) {
        self.sdl_renderer.set_size(size)
    }

    fn set_window_title<S: AsRef<str>>(&mut self, title: S) {
        self.sdl_renderer.set_window_title(title)
    }

    fn set_draw_origin(&mut self, new_origin: Vec2) {
        self.origin = new_origin;
    }
}
