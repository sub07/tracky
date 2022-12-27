use crate::model::patterns::Patterns;
use crate::renderer::Renderer;
use crate::view::Draw;
use crate::view::pattern::PatternDrawData;

impl Draw for Patterns {
    fn draw(&self, renderer: &mut Renderer, mut x: i32, y: i32, _: ()) {
        let selected_pattern_index = self.cursor_x / 4;
        for (pattern_index, pattern) in self.iter().enumerate() {
            let local_x_cursor = if selected_pattern_index == pattern_index {
                Some(self.cursor_x % 4)
            } else {
                None
            };
            pattern.draw(renderer, x, y, PatternDrawData::new(local_x_cursor, self.cursor_y));
            x += renderer.glyph_width() as i32 * 7;
        }
    }
}
