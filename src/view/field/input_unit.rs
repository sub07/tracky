use crate::model::field::input_unit::InputUnit;
use crate::mono_font_atlas::TextAlignment;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::Draw;

impl Draw for InputUnit {
    type DrawData = bool;

    fn draw(&self, renderer: &mut Renderer, x: i32, y: i32, theme: &Theme, selected: bool) {
        if selected {
            renderer.draw_text_with_background(format!("{}", self.value), x, y, theme.selected_text_color(), theme.selected_background_color(), TextAlignment::Left);
        } else {
            renderer.draw_text(format!("{}", self.value), x, y, theme.text_color(), TextAlignment::Left)
        }
    }
}
