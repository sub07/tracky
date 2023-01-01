use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::color::Color;
use crate::font_atlas::{FontAtlas, TextAlignment};

pub struct Renderer<'a> {
    canvas: WindowCanvas,
    font: FontAtlas<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new<P: AsRef<Path>>(canvas: WindowCanvas, texture_creator: &'a TextureCreator<WindowContext>, default_font_path: P, default_font_size: u16, default_font_glyphs: &'static str) -> Renderer<'a> {
        let font = FontAtlas::new(texture_creator, default_font_path, default_font_size, default_font_glyphs);

        Renderer {
            canvas,
            font,
        }
    }

    pub fn clear<C: Color>(&mut self, color: C) {
        self.canvas.set_draw_color(color.into_sdl_color());
        self.canvas.clear();
    }

    pub fn draw_text<S: AsRef<str>, C: Color>(&mut self, text: S, x: i32, y: i32, color: C, alignment: TextAlignment) {
        self.font.draw(&mut self.canvas, text, x, y, color.into_sdl_color(), alignment);
    }

    pub fn draw_text_with_background<S: AsRef<str>, C1: Color, C2: Color>(&mut self, text: S, x: i32, y: i32, foreground_color: C1, background_color: C2, alignment: TextAlignment) {
        self.font.draw_with_background(&mut self.canvas, text, x, y, foreground_color.into_sdl_color(), background_color.into_sdl_color(), alignment);
    }

    pub fn draw_rect<C: Color>(&mut self, x: i32, y: i32, w: u32, h: u32, color: C) {
        self.canvas.set_draw_color(color.into_sdl_color());
        self.canvas.fill_rect(Rect::new(x, y, w as u32, h as u32)).unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn glyph_width(&self) -> u32 {
        self.font.glyph_width()
    }

    pub fn glyph_height(&self) -> u32 {
        self.font.glyph_height()
    }
}
