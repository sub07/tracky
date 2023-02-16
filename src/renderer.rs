use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::{Scalar, Vec2};
use crate::mono_font_atlas::{MonoFontAtlas, TextStyle};

pub trait Renderer {
    fn clear(&mut self, color: (u8, u8, u8));
    fn set_clip(&mut self, x: Scalar, y: Scalar, w: Scalar, h: Scalar);
    fn draw_text(&mut self, text: &str, x: Scalar, y: Scalar, text_style: TextStyle);
    fn draw_rect(&mut self, x: Scalar, y: Scalar, w: Scalar, h: Scalar, color: (u8, u8, u8), filled: bool);
    fn present(&mut self);
    fn glyph_size(&self) -> Vec2;
    fn window_size(&self) -> Vec2;
    #[deprecated]
    fn glyph_width(&self) -> Scalar;
    #[deprecated]
    fn glyph_height(&self) -> Scalar;
    fn set_window_size(&mut self, w: Scalar, h: Scalar);
    fn set_window_title(&mut self, title: &str);
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

impl Renderer for SdlRenderer<'_> {
    fn clear(&mut self, color: (u8, u8, u8)) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn set_clip(&mut self, x: Scalar, y: Scalar, w: Scalar, h: Scalar) {
        self.canvas.set_clip_rect(Rect::new(x, y, w as u32, h as u32));
    }

    fn draw_text(&mut self, text: &str, x: Scalar, y: Scalar, text_style: TextStyle) {
        self.font.draw(&mut self.canvas, text, x, y, text_style);
    }

    fn draw_rect(&mut self, x: Scalar, y: Scalar, w: Scalar, h: Scalar, color: (u8, u8, u8), filled: bool) {
        self.canvas.set_draw_color(color);
        let rect = Rect::new(x, y, w as u32, h as u32);
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
        [w as Scalar, h as Scalar].into()
    }

    fn glyph_width(&self) -> Scalar {
        self.glyph_size()[0]
    }

    fn glyph_height(&self) -> Scalar {
        self.glyph_size()[1]
    }

    fn set_window_size(&mut self, w: Scalar, h: Scalar) {
        self.canvas.window_mut().set_size(w as u32, h as u32).unwrap();
    }

    fn set_window_title(&mut self, title: &str) {
        self.canvas.window_mut().set_title(title.as_ref()).unwrap();
    }
}
