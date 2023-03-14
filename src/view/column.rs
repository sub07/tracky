use rust_utils_macro::New;

use crate::model::pattern::column::Column;
use crate::model::pattern::ColumnLineElement;
use crate::rendering::renderer::Renderer;

use crate::theme::Theme;
use crate::view::column_line::ColumnLineDrawData;
use crate::view::Draw;

#[derive(New)]
pub struct ColumnDrawData {
    local_x_cursor: i32,
    cursor_y: i32,
    line_offset: i32,
}

impl Draw for Column {
    type DrawData = ColumnDrawData;

    fn draw<R: Renderer>(&self, renderer: &mut R, x: i32, mut y: i32, theme: &Theme, ColumnDrawData { local_x_cursor, cursor_y, line_offset }: ColumnDrawData) {
        let nb_line = self.len() - line_offset;
        let pattern_background_width = renderer.glyph_size().w() * (ColumnLineElement::LINE_LEN + ColumnLineElement::SIZE as i32 - 1);
        let pattern_background_height = renderer.glyph_size().h() * nb_line;
        renderer.draw_rect([x, y].into(), [pattern_background_width, pattern_background_height].into(), theme.pattern_background_color, true);
        for (line_index, line) in self.iter().enumerate().skip(line_offset as usize) {
            line.draw(renderer, x, y, theme, ColumnLineDrawData::new(line_index as i32 == cursor_y, local_x_cursor));
            y += renderer.glyph_size().h();
        }
    }
}
