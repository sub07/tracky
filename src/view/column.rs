use rust_utils_macro::New;

use crate::model::column::Column;
use crate::model::ColumnLineElement;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::column_line::ColumnLineDrawData;
use crate::view::Draw;

#[derive(New)]
pub struct ColumnDrawData {
    local_x_cursor: i32,
    cursor_y: i32,
    from_line: i32,
}

impl Draw for Column {
    type DrawData = ColumnDrawData;

    fn draw(&self, renderer: &mut Renderer, x: i32, mut y: i32, theme: &Theme, ColumnDrawData { local_x_cursor, cursor_y, from_line }: ColumnDrawData) {
        let nb_line = self.len() - from_line;
        let pattern_background_width = renderer.glyph_width() * (ColumnLineElement::LINE_LEN + ColumnLineElement::SIZE as i32 - 1);
        let pattern_background_height = renderer.glyph_height() * nb_line;
        renderer.draw_rect(x, y, pattern_background_width, pattern_background_height, theme.pattern_background_color());
        for (line_index, line) in self.iter().enumerate().skip(from_line as usize) {
            line.draw(renderer, x, y, theme, ColumnLineDrawData::new(line_index as i32 == cursor_y, local_x_cursor));
            y += renderer.glyph_height();
        }
    }
}
