use derive_new::new;

use crate::model::pattern::Pattern;
use crate::renderer::Renderer;
use crate::view::Draw;
use crate::view::pattern_line::PatternLineDrawData;

#[derive(new)]
pub struct PatternDrawData {
    local_x_cursor: Option<usize>,
    cursor_y: usize,
}

impl Draw for Pattern {
    type DrawData = PatternDrawData;

    fn draw(&self, renderer: &mut Renderer, x: i32, mut y: i32, PatternDrawData { local_x_cursor, cursor_y }: PatternDrawData) {
        if let Some(index) = local_x_cursor && index > 3 {
            panic!("Invalid local index: {x}");
        }
        let pattern_background_width = renderer.glyph_width() * 6;
        let pattern_background_height = renderer.glyph_height() * self.len() as u32;
        renderer.draw_rect(x, y, pattern_background_width, pattern_background_height, (40, 40, 60));
        for (line_index, line) in self.iter().enumerate() {
            line.draw(renderer, x, y, PatternLineDrawData::new(line_index == cursor_y, local_x_cursor));
            y += renderer.glyph_height() as i32;
        }
    }
}
