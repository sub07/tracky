use rust_utils_macro::New;

use crate::model::pattern::patterns::Patterns;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::Draw;
use crate::view::pattern::PatternDrawData;

#[derive(New)]
pub struct PatternsDrawData {}

impl Draw for Patterns {
    type DrawData = PatternsDrawData;

    fn draw<R: Renderer>(&self, renderer: &mut R, x: i32, mut y: i32, theme: &Theme, PatternsDrawData {}: PatternsDrawData) {
        renderer.draw_text(
            &format!("Pattern {}/{}", self.selected_pattern_index + 1, self.nb_patterns()),
            x, y,
            theme.pattern_index_style(),
        );
        y += renderer.glyph_height();
        self.current_pattern().draw(renderer, x, y, theme, PatternDrawData::new(self.cursor_x, self.cursor_y, 0, 0));
    }
}
