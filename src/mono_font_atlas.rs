use std::collections::HashMap;
use std::path::Path;

use rust_utils_macro::New;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::video::WindowContext;

pub enum TextAlignment {
    Left,
    Right,
}

#[derive(New)]
pub struct TextStyle {
    foreground_color: (u8, u8, u8),
    background_color: Option<(u8, u8, u8)>,
    alignment: TextAlignment,
}

pub struct MonoFontAtlas<'a> {
    texture: Texture<'a>,
    glyph_map: HashMap<char, i32>,
    glyph_width: u32,
    glyph_height: u32,
}

impl<'a> MonoFontAtlas<'a> {
    pub fn new<P: AsRef<Path>>(texture_creator: &'a TextureCreator<WindowContext>, path: P, font_size: u16, glyphs: &str) -> MonoFontAtlas<'a> {
        let ttf = sdl2::ttf::init().unwrap();
        let font = ttf.load_font(path, font_size).unwrap();
        assert!(font.face_is_fixed_width());
        let font_atlas = font.render(glyphs).blended(Color::RGB(255, 255, 255)).unwrap();
        let texture = texture_creator.create_texture_from_surface(font_atlas).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let mut glyph_map = HashMap::with_capacity(glyphs.len());
        let glyph_width = width / glyphs.len() as u32;
        for (index, glyph) in glyphs.chars().enumerate() {
            glyph_map.insert(glyph, (index * glyph_width as usize) as i32);
        }
        MonoFontAtlas {
            texture,
            glyph_map,
            glyph_width,
            glyph_height: height,
        }
    }

    pub fn draw<S: AsRef<str>>(&mut self, renderer: &mut WindowCanvas, text: S, x: i32, y: i32, style: TextStyle) {
        let cur_x = match style.alignment {
            TextAlignment::Left => x,
            TextAlignment::Right => x - (text.as_ref().len() as u32 * self.glyph_width) as i32,
        };

        if let Some(background_color) = &style.background_color {
            let width = text.as_ref().len() as u32 * self.glyph_width;
            let height = self.glyph_height;
            renderer.set_draw_color(*background_color);
            renderer.fill_rect(Rect::new(cur_x, y, width, height)).unwrap();
        }

        let foreground_color: Color = style.foreground_color.into();
        self.texture.set_color_mod(foreground_color.r, foreground_color.g, foreground_color.b);

        let mut cur_x = cur_x;
        for glyph in text.as_ref().chars() {
            if glyph == ' ' {
                cur_x += self.glyph_width as i32;
                continue;
            }
            let src_x = *self.glyph_map.get(&glyph).unwrap_or_else(|| panic!("Glyph {glyph} is not supported"));
            renderer.copy(
                &self.texture,
                Some(Rect::new(src_x, 0, self.glyph_width, self.glyph_height)),
                Some(Rect::new(cur_x, y, self.glyph_width, self.glyph_height)),
            ).unwrap();
            cur_x += self.glyph_width as i32;
        }
    }

    pub fn glyph_width(&self) -> i32 {
        self.glyph_width as i32
    }

    pub fn glyph_height(&self) -> i32 {
        self.glyph_height as i32
    }
}
