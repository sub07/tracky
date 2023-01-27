use std::cell::RefCell;
use std::rc::Rc;
use rust_utils_macro::New;
use crate::model::patterns::Patterns;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::Draw;
use crate::view::pattern::PatternDrawData;

#[derive(New)]
pub struct PatternsView {
    pub model: Rc<RefCell<Patterns>>,
    #[new_default]
    pub x_offset: i32,
    #[new_default]
    pub y_offset: i32,
    #[new_default]
    pub last_x_cursor: i32,
    #[new_default]
    pub last_y_cursor: i32,
}

impl Draw for PatternsView {
    fn draw(&self, renderer: &mut Renderer, x: i32, mut y: i32, theme: &Theme, _: ()) {
        let model = self.model.borrow();
        renderer.draw_text(
            format!("Pattern {}/{}", model.selected_pattern_index + 1, model.nb_patterns()),
            x, y,
            theme.pattern_index_style(),
        );
        y += renderer.glyph_height();
        model.current_pattern().draw(renderer, x, y, theme, PatternDrawData::new(model.cursor_x, model.cursor_y));
    }
}