use derive_new::new;

use crate::model::field::velocity::VelocityField;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::Draw;
use crate::view::field::draw_char_input_unit;

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

        let (vel_char_1, vel_char_2) = match self.value {
            None => ('.', '.'),
            Some(velocity) => {
                (
                    char::from_digit((velocity >> 4) as u32, 16).unwrap().to_ascii_uppercase(),
                    char::from_digit((velocity & 0x0F) as u32, 16).unwrap().to_ascii_uppercase(),
                )
            }
        };

        draw_char_input_unit(renderer, x, y, theme, index == 0, vel_char_1);
        x += renderer.glyph_width() as i32;
        draw_char_input_unit(renderer, x, y, theme, index == 1, vel_char_2);
    }
}