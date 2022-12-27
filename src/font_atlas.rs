use std::collections::HashMap;
use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::video::WindowContext;

pub enum TextAlignment {
    Left,
    Right,
}

pub struct FontAtlas<'a> {
    texture: Texture<'a>,
    glyph_map: HashMap<char, i32>,
    glyph_width: u32,
    glyph_height: u32,
}

impl<'a> FontAtlas<'a> {
    pub fn new<P: AsRef<Path>>(texture_creator: &'a TextureCreator<WindowContext>, path: P, font_size: u16, glyphs: &str) -> FontAtlas<'a> {
        let ttf = sdl2::ttf::init().unwrap();
        let font = ttf.load_font(path, font_size).unwrap();
        let font_atlas = font.render(glyphs).blended(Color::RGB(255, 255, 255)).unwrap();
        let texture = texture_creator.create_texture_from_surface(font_atlas).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let mut glyph_map = HashMap::with_capacity(glyphs.len());
        let glyph_width = width / glyphs.len() as u32;
        for (index, glyph) in glyphs.chars().enumerate() {
            glyph_map.insert(glyph, (index * glyph_width as usize) as i32);
        }
        FontAtlas {
            texture,
            glyph_map,
            glyph_width,
            glyph_height: height,
        }
    }

    pub fn draw_with_background<S: AsRef<str>>(&mut self, renderer: &mut WindowCanvas, text: S, x: i32, y: i32, foreground_color: Color, background_color: Color, alignment: TextAlignment) {
        let offset = match alignment {
            TextAlignment::Left => 0,
            TextAlignment::Right => (text.as_ref().len() as u32 * self.glyph_width) as i32
        };
        let x_offset = x - offset;
        let width = text.as_ref().len() as u32 * self.glyph_width;
        let height = self.glyph_height;
        renderer.set_draw_color(background_color);
        renderer.fill_rect(Rect::new(x_offset, y, width, height)).unwrap();
        self.draw(renderer, text, x, y, foreground_color, alignment);
    }

    pub fn draw<S: AsRef<str>>(&mut self, renderer: &mut WindowCanvas, text: S, x: i32, y: i32, color: Color, alignment: TextAlignment) {
        self.texture.set_color_mod(color.r, color.g, color.b);
        let offset = match alignment {
            TextAlignment::Left => 0,
            TextAlignment::Right => (text.as_ref().len() as u32 * self.glyph_width) as i32
        };
        let mut dest_x = x - offset;
        for glyph in text.as_ref().chars() {
            if glyph == ' ' {
                dest_x += self.glyph_width as i32;
                continue;
            }
            let src_x = *self.glyph_map.get(&glyph).expect(&format!("Glyph {glyph} is not supported"));
            renderer.copy(
                &self.texture,
                Some(Rect::new(src_x, 0, self.glyph_width, self.glyph_height)),
                Some(Rect::new(dest_x, y, self.glyph_width, self.glyph_height)),
            ).unwrap();
            dest_x += self.glyph_width as i32;
        }
    }

    pub fn glyph_width(&self) -> u32 {
        self.glyph_width
    }

    pub fn glyph_height(&self) -> u32 {
        self.glyph_height
    }
}

pub trait TextureAtlas {
    fn draw(&self, renderer: &mut WindowCanvas, index: usize, atlas_size: usize, at: (i32, i32));
}

impl TextureAtlas for Texture<'_> {
    fn draw(&self, renderer: &mut WindowCanvas, index: usize, atlas_size: usize, at: (i32, i32)) {
        let TextureQuery { width, height, .. } = self.query();
        let width = width / atlas_size as u32;
        renderer.copy(
            self,
            Some(Rect::new((width * index as u32) as i32, 0, width, height)),
            Some(Rect::new(at.0, at.1, width, height)),
        ).unwrap();
    }
}
