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
    pub cursor_x: i32,
    pub cursor_y: i32,
}

impl Draw for Pattern {
    type DrawData = PatternDrawData;

    fn draw(&self, renderer: &mut Renderer, mut x: i32, mut y: i32, theme: &Theme, PatternDrawData { cursor_x, cursor_y }: PatternDrawData) {
        let gh = renderer.glyph_height();
        let gw = renderer.glyph_width();
        let height = renderer.height();
        let width = renderer.width();
        y += gh;
        x += gw * 4;
        let below_cursor_y_pix = y + cursor_y * gh + gh;
        let from_line = if below_cursor_y_pix > height {
            (below_cursor_y_pix - height) / gh
        } else { 0 };
        let mut line_num_y = y;
        for i in from_line..self.column_len() {
            let line = format!("{i} ");
            if i == cursor_y {
                renderer.draw_text_with_background(line, x, line_num_y, theme.text_color(), theme.highlighted_background_color(), TextAlignment::Right);
            } else {
                renderer.draw_text(line, x, line_num_y, theme.text_color(), TextAlignment::Right);
            }
            line_num_y += renderer.glyph_height();
        }

        let col_width = (ColumnLineElement::LINE_LEN + ColumnLineElement::SIZE as i32) * renderer.glyph_width();
        let col_index = cursor_x / ColumnLineElement::LINE_LEN;
        let right_cursor_col = x + col_index * col_width + col_width;

        let from_col = if right_cursor_col > width {
            ((right_cursor_col - width) / col_width + 1) as usize
        } else { 0 };

        for (pattern_index, column) in self.iter().enumerate().skip(from_col) {
            renderer.draw_text_with_background(format!("{}", pattern_index + 1), x, y - renderer.glyph_height(), theme.text_color(), theme.pattern_background_color(), TextAlignment::Left);
            let local_x_cursor = {
                let cursor_x = cursor_x;
                let pattern_index = pattern_index as i32;
                cursor_x - pattern_index * ColumnLineElement::LINE_LEN
            };
            column.draw(renderer, x, y, theme, ColumnDrawData::new(local_x_cursor, cursor_y, from_line));
            x += renderer.glyph_width() * (ColumnLineElement::LINE_LEN + 2);
        }
    }
}
