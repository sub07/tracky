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
        let gh = renderer.glyph_height() as i32;
        let gw = renderer.glyph_width() as i32;
        let height = renderer.height() as i32;
        y += gh;
        x += gw * 4;
        let below_cursor_y_pix = y + cursor_y as i32 * gh + gh;
        let from_line = if below_cursor_y_pix > height {
            let beyond = below_cursor_y_pix - height;
            (beyond / gh)
        } else { 0 };
        let from_line = from_line as usize;
        let mut line_num_y = y;
        for i in from_line..self.column_len() {
            let line = format!("{i} ");
            if i == cursor_y {
                renderer.draw_text_with_background(line, x, line_num_y, theme.text_color(), theme.highlighted_background_color(), TextAlignment::Right);
            } else {
                renderer.draw_text(line, x, line_num_y, theme.text_color(), TextAlignment::Right);
            }
            line_num_y += renderer.glyph_height() as i32;
        }
        for (pattern_index, column) in self.iter().enumerate() {
            renderer.draw_text_with_background(format!("{}", pattern_index + 1), x, y - renderer.glyph_height() as i32, theme.text_color(), theme.pattern_background_color(), TextAlignment::Left);
            let local_x_cursor = {
                let cursor_x = cursor_x as i32;
                let pattern_index = pattern_index as i32;
                cursor_x - pattern_index * ColumnLineElement::LINE_LEN as i32
            };
            column.draw(renderer, x, y, theme, ColumnDrawData::new(local_x_cursor, cursor_y, from_line));
            x += renderer.glyph_width() as i32 * (ColumnLineElement::LINE_LEN + 2) as i32;
        }
    }
}
