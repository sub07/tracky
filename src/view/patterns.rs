use crate::model::PatternLineElement;
use crate::model::patterns::Patterns;
use crate::renderer::Renderer;
use crate::view::Draw;
use crate::view::pattern::PatternDrawData;

impl Draw for Patterns {
    fn draw(&self, renderer: &mut Renderer, mut x: i32, y: i32, _: ()) {
        for (pattern_index, pattern) in self.iter().enumerate() {
            let local_x_cursor = {
                let cursor_x = self.cursor_x as i32;
                let pattern_index = pattern_index as i32;
                cursor_x - pattern_index * PatternLineElement::LINE_LEN as i32
            };
            pattern.draw(renderer, x, y, PatternDrawData::new(local_x_cursor, self.cursor_y));
            x += renderer.glyph_width() as i32 * (PatternLineElement::LINE_LEN + 2) as i32;
        }
    }
}
