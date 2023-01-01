use derive_new::new;

use crate::font_atlas::TextAlignment;
use crate::model::PatternLineElement;
use crate::model::patterns::Patterns;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::Draw;
use crate::view::pattern::PatternDrawData;

impl Draw for Patterns {
    fn draw(&self, renderer: &mut Renderer, mut x: i32, y: i32, theme: &Theme, _: ()) {
        x += (renderer.glyph_width() * 4) as i32;
        for i in 0..self.pattern_len() {
            let y = y + (i as u32 * renderer.glyph_height()) as i32;
            let line = format!("{i} ");
            if i == self.cursor_y {
                renderer.draw_text_with_background(line, x, y, theme.text_color(), theme.highlighted_background_color(), TextAlignment::Right);
            } else {
                renderer.draw_text(line, x, y, theme.text_color(), TextAlignment::Right);
            }
        }
        for (pattern_index, pattern) in self.iter().enumerate() {
            let local_x_cursor = {
                let cursor_x = self.cursor_x as i32;
                let pattern_index = pattern_index as i32;
                cursor_x - pattern_index * PatternLineElement::LINE_LEN as i32
            };
            pattern.draw(renderer, x, y, theme, PatternDrawData::new(local_x_cursor, self.cursor_y));
            x += renderer.glyph_width() as i32 * (PatternLineElement::LINE_LEN + 2) as i32;
        }
    }
}
