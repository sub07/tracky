use crate::model::patterns::Patterns;
use crate::mono_font_atlas::TextAlignment;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::Draw;
use crate::view::pattern::PatternDrawData;

impl Draw for Patterns {
    fn draw(&self, renderer: &mut Renderer, x: i32, mut y: i32, theme: &Theme, _: ()) {
        renderer.draw_text_with_background(format!("Pattern {}/{}", self.selected_pattern_index + 1, self.nb_patterns()), x, y, theme.text_color(), theme.pattern_background_color(), TextAlignment::Left);
        y += renderer.glyph_height() as i32;
        self.current_pattern().draw(renderer, x, y, theme, PatternDrawData::new(self.cursor_x, self.cursor_y));
    }
}