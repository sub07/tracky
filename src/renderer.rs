use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::mono_font_atlas::{MonoFontAtlas, TextStyle};

pub struct Renderer<'a> {
    canvas: WindowCanvas,
    font: MonoFontAtlas<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new<P: AsRef<Path>>(canvas: WindowCanvas, texture_creator: &'a TextureCreator<WindowContext>, default_font_path: P, default_font_size: u16, default_font_glyphs: &'static str) -> Renderer<'a> {
        let font = MonoFontAtlas::new(texture_creator, default_font_path, default_font_size, default_font_glyphs);
        Renderer { canvas, font }
    }

    pub fn clear(&mut self, color: (u8, u8, u8)) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn draw_text<S: AsRef<str>>(&mut self, text: S, x: i32, y: i32, text_style: TextStyle) {
        self.font.draw(&mut self.canvas, text, x, y, text_style);
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: (u8, u8, u8)) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(x, y, w as u32, h as u32)).unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn glyph_width(&self) -> i32 {
        self.font.glyph_width()
    }

    pub fn glyph_height(&self) -> i32 {
        self.font.glyph_height()
    }

    pub fn width(&self) -> i32 { self.canvas.output_size().unwrap().0 as i32 }

    pub fn height(&self) -> i32 { self.canvas.output_size().unwrap().1 as i32 }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.canvas.window_mut().set_size(width as u32, height as u32).unwrap()
    }

    pub fn set_window_title<S: AsRef<str>>(&mut self, title: S) {
        self.canvas.window_mut().set_title(title.as_ref()).unwrap();
    }
}
