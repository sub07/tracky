use rust_utils_macro::New;

use crate::model::pattern::ColumnLineElement;
use crate::model::pattern::pattern::Pattern;
use crate::renderer::{WindowRenderer};
use crate::theme::Theme;
use crate::Vec2;
use crate::view::column::ColumnDrawData;
use crate::view::Draw;

#[derive(New)]
pub struct PatternDrawData {
    cursor_x: i32,
    cursor_y: i32,
    line_offset: i32,
    column_offset: i32,
}

impl Draw for Pattern {
    type DrawData = PatternDrawData;

    fn draw<Renderer: WindowRenderer>(&self, renderer: &mut Renderer, mut x: i32, mut y: i32, theme: &Theme, PatternDrawData { cursor_x, cursor_y, line_offset, column_offset }: PatternDrawData) {
        let gh = renderer.glyph_height();
        let gw = renderer.glyph_width();
        y += gh;
        x += gw * 4;
        let mut line_num_y = y;
        for i in line_offset..self.column_len() {
            renderer.draw_text(
                format!("{i} "),
                Vec2::new(x, line_num_y),
                if i == cursor_y { theme.selected_line_count_style() } else { theme.line_count_style() },
            );
            line_num_y += gh;
        }

        for (pattern_index, column) in self.iter().enumerate().skip(column_offset as usize) {
            renderer.draw_text(
                format!("{}", pattern_index + 1),
                Vec2::new(x, y - gh),
                theme.column_number_style(),
            );
            let local_x_cursor = {
                let cursor_x = cursor_x;
                let pattern_index = pattern_index as i32;
                cursor_x - pattern_index * ColumnLineElement::LINE_LEN
            };
            column.draw(renderer, x, y, theme, ColumnDrawData::new(local_x_cursor, cursor_y, line_offset));
            x += gw * (ColumnLineElement::LINE_LEN + 2);
        }
    }
}
