use crate::font_atlas::TextAlignment;
use crate::model::field::input_unit::InputUnit;
use crate::renderer::Renderer;
use crate::view::Draw;

impl Draw for InputUnit {
    type DrawData = bool;

    fn draw(&self, renderer: &mut Renderer, x: i32, y: i32, selected: bool) {
        if selected {
            renderer.draw_text_with_background(format!("{}", self.value), x, y, (0, 0, 0), (255, 255, 255), TextAlignment::Left);
        } else {
            renderer.draw_text(format!("{}", self.value), x, y, (255, 255, 255), TextAlignment::Left)
        }
    }
}
