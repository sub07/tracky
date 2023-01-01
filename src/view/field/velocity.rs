use derive_new::new;

use crate::model::field::velocity::VelocityField;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::Draw;

#[derive(new)]
pub struct VelocityFieldDrawData {
    local_x_selected: Option<i32>,
}

impl Draw for VelocityField {
    type DrawData = VelocityFieldDrawData;

    fn draw(&self, renderer: &mut Renderer, mut x: i32, y: i32, theme: &Theme, VelocityFieldDrawData { local_x_selected }: VelocityFieldDrawData) {
        let index = match local_x_selected {
            Some(index) => index,
            None => -1,
        };
        self.digit1.draw(renderer, x, y, theme, index == 0);
        x += renderer.glyph_width() as i32;
        self.digit2.draw(renderer, x, y, theme, index == 1);
    }
}
