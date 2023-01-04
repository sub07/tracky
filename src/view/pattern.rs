use derive_new::new;

use crate::model::ColumnLineElement;
use crate::model::pattern::Pattern;
use crate::mono_font_atlas::TextAlignment;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::column::ColumnDrawData;
use crate::view::Draw;

#[derive(new)]
pub struct PatternDrawData {
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl Draw for Pattern {
    type DrawData = PatternDrawData;

    fn draw(&self, renderer: &mut Renderer, mut x: i32, mut y: i32, theme: &Theme, PatternDrawData { cursor_x, cursor_y }: PatternDrawData) {
        y += renderer.glyph_height() as i32;
        x += (renderer.glyph_width() * 4) as i32;
        for i in 0..self.column_len() {
            let y = y + (i as u32 * renderer.glyph_height()) as i32;
            let line = format!("{i} ");
            if i == cursor_y {
                renderer.draw_text_with_background(line, x, y, theme.text_color(), theme.highlighted_background_color(), TextAlignment::Right);
            } else {
                renderer.draw_text(line, x, y, theme.text_color(), TextAlignment::Right);
            }
        }
        for (pattern_index, column) in self.iter().enumerate() {
            renderer.draw_text_with_background(format!("{}", pattern_index + 1), x, y - renderer.glyph_height() as i32, theme.text_color(), theme.pattern_background_color(), TextAlignment::Left);
            let local_x_cursor = {
                let cursor_x = cursor_x as i32;
                let pattern_index = pattern_index as i32;
                cursor_x - pattern_index * ColumnLineElement::LINE_LEN as i32
            };
            column.draw(renderer, x, y, theme, ColumnDrawData::new(local_x_cursor, cursor_y));
            x += renderer.glyph_width() as i32 * (ColumnLineElement::LINE_LEN + 2) as i32;
        }
    }
}
