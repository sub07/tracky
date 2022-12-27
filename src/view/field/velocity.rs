use derive_new::new;

use crate::model::field::velocity::VelocityField;
use crate::renderer::Renderer;
use crate::view::Draw;

#[derive(new)]
pub struct VelocityFieldDrawData {
    local_x_selected: Option<usize>,
}

impl Draw for VelocityField {
    type DrawData = VelocityFieldDrawData;

    fn draw(&self, renderer: &mut Renderer, mut x: i32, y: i32, VelocityFieldDrawData { local_x_selected }: VelocityFieldDrawData) {
        let index = match local_x_selected {
            Some(x) if x <= 1 => x,
            None => usize::MAX,
            Some(x) => panic!("Invalid local index: {x}"),
        };
        self.digit1.draw(renderer, x, y, index == 0);
        x += renderer.glyph_width() as i32;
        self.digit2.draw(renderer, x, y, index == 1);
    }
}
