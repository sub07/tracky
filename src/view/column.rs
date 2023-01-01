use derive_new::new;

use crate::model::ColumnLineElement;
use crate::model::column::Column;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::Draw;
use crate::view::column_line::ColumnLineDrawData;

#[derive(new)]
pub struct ColumnDrawData {
    local_x_cursor: i32,
    cursor_y: usize,
}

impl Draw for Column {
    type DrawData = ColumnDrawData;

    fn draw(&self, renderer: &mut Renderer, x: i32, mut y: i32, theme: &Theme, ColumnDrawData { local_x_cursor, cursor_y }: ColumnDrawData) {
        let pattern_background_width = renderer.glyph_width() * (ColumnLineElement::LINE_LEN + ColumnLineElement::NB_VARIANT - 1) as u32;
        let pattern_background_height = renderer.glyph_height() * self.len() as u32;
        renderer.draw_rect(x, y, pattern_background_width, pattern_background_height, theme.pattern_background_color());
        for (line_index, line) in self.iter().enumerate() {
            line.draw(renderer, x, y, theme, ColumnLineDrawData::new(line_index == cursor_y, local_x_cursor));
            y += renderer.glyph_height() as i32;
        }
    }
}
