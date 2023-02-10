use rust_utils_macro::New;

use crate::model::pattern::patterns::Patterns;
use crate::renderer::WindowRenderer;
use crate::theme::Theme;
use crate::Vec2;
use crate::view::{Draw, ViewDraw};
use crate::view::pattern::PatternDrawData;

#[derive(New)]
struct View<ViewData> {
    position: Vec2,
    size: Vec2,
    data: ViewData,
}

#[derive(New)]
struct ColumnLineViewData {
    chars: Vec<char>,
}

impl ViewDraw for View<ColumnLineViewData> {
    fn draw<Renderer: WindowRenderer>(&self, renderer: &mut Renderer, theme: &Theme) {}
}

#[derive(New)]
struct ColumnViewData {
    lines: Vec<ColumnLineViewData>,
}

#[derive(New)]
struct PatternViewData {
    columns: Vec<ColumnViewData>,
}

#[derive(New)]
struct PatternsViewData {
    line_offset: i32,
    column_offset: i32,
    current_pattern: PatternViewData,
    current_pattern_index: i32,
    nb_pattern: i32,
}

#[derive(New)]
pub struct PatternsDrawData {}

impl Draw for Patterns {
    type DrawData = PatternsDrawData;

    fn draw<Renderer: WindowRenderer>(&self, renderer: &mut Renderer, x: i32, mut y: i32, theme: &Theme, PatternsDrawData {}: PatternsDrawData) {
        renderer.draw_text(
            format!("Pattern {}/{}", self.selected_pattern_index + 1, self.nb_patterns()),
            Vec2::new(x, y),
            theme.pattern_index_style(),
        );
        y += renderer.glyph_height();
        self.current_pattern().draw(renderer, x, y, theme, PatternDrawData::new(self.cursor_x, self.cursor_y, 0, 0));
    }
}
