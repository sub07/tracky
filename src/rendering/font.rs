use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings};
use rust_utils::iter::zip_self::ZipSelf;
use rust_utils_macro::New;
use tiny_skia::{Pixmap, PixmapPaint};
use tiny_skia_path::{IntSize, Transform};

use crate::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub enum TextAlignment {
    Left,
    Right,
}

#[derive(New, Clone, Copy)]
pub struct TextStyle {
    pub foreground_color: (u8, u8, u8),
    pub background_color: Option<(u8, u8, u8)>,
    pub alignment: TextAlignment,
}

pub struct Font {
    inner: fontdue::Font,
    pub font_size: f32,
    pub glyph_size: Vec2,
}

impl Font {
    pub fn from_bytes(bytes: &[u8], font_size: f32) -> Font {
        let font = fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default()).unwrap();

        let glyph_width = font.metrics('.', font_size).advance_width.ceil() as i32;
        let glyph_height = font.horizontal_line_metrics(font_size).unwrap().new_line_size.ceil() as i32;

        Font {
            inner: font,
            font_size,
            glyph_size: Vec2::new(glyph_width, glyph_height),
        }
    }

    pub fn draw_text_on<S: AsRef<str>>(&self, pixmap: &mut Pixmap, text: S, position: Vec2, text_style: TextStyle) {
        assert!(!text.as_ref().contains('\n') && !text.as_ref().contains('\r'));
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        let layout_settings = LayoutSettings {
            x: position.x() as f32,
            y: position.y() as f32,
            max_width: None,
            ..Default::default()
        };
        layout.reset(&layout_settings);
        layout.append(&[&self.inner], &fontdue::layout::TextStyle::new(text.as_ref(), self.font_size, 0));
        for glyph_info in layout.glyphs() {
            let (_, bitmap) = self.inner.rasterize(glyph_info.parent, self.font_size);
            if bitmap.is_empty() {
                continue;
            }
            let mut pixels = bitmap.iter().cloned().zip_self(4).collect::<Vec<_>>();
            for [r, g, b, _] in pixels.as_chunks_mut::<4>().0 {
                let temp_r = *r as f64;
                let temp_g = *g as f64;
                let temp_b = *b as f64;

                *r = ((text_style.foreground_color.0 as f64 / 255.0) * temp_r) as u8;
                *g = ((text_style.foreground_color.1 as f64 / 255.0) * temp_g) as u8;
                *b = ((text_style.foreground_color.2 as f64 / 255.0) * temp_b) as u8;
            }
            let glyph_pixmap = Pixmap::from_vec(
                pixels,
                IntSize::from_wh(
                    glyph_info.width as u32,
                    glyph_info.height as u32,
                ).unwrap(),
            ).unwrap();

            pixmap.draw_pixmap(glyph_info.x as i32, glyph_info.y as i32, glyph_pixmap.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
        }
    }
}
