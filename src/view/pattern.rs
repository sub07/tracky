use rust_utils_macro::New;

use crate::model::pattern::ColumnLineElement;
use crate::model::pattern::pattern::Pattern;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::column::ColumnDrawData;
use crate::view::Draw;

#[derive(New)]
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
            renderer.draw_text(
                format!("{i} "),
                x, line_num_y,
                if i == cursor_y { theme.selected_line_count_style() } else { theme.line_count_style() },
            );
            line_num_y += gh;
        }

        let col_width = (ColumnLineElement::LINE_LEN + ColumnLineElement::SIZE as i32) * gw;
        let col_index = cursor_x / ColumnLineElement::LINE_LEN;
        let right_cursor_col = x + col_index * col_width + col_width;

        let from_col = if right_cursor_col > width {
            ((right_cursor_col - width) / col_width + 1) as usize
        } else { 0 };

        for (pattern_index, column) in self.iter().enumerate().skip(from_col) {
            renderer.draw_text(
                format!("{}", pattern_index + 1),
                x, y - gh,
                theme.column_number_style(),
            );
            let local_x_cursor = {
                let cursor_x = cursor_x;
                let pattern_index = pattern_index as i32;
                cursor_x - pattern_index * ColumnLineElement::LINE_LEN
            };
            column.draw(renderer, x, y, theme, ColumnDrawData::new(local_x_cursor, cursor_y, from_line));
            x += gw * (ColumnLineElement::LINE_LEN + 2);
        }
    }
}
