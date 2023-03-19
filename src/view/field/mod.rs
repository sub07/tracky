use crate::Scalar;
use crate::rendering::renderer::Renderer;

use crate::theme::Theme;

pub mod note;
pub mod velocity;

pub fn draw_char_input_unit<R: Renderer>(renderer: &mut R, x: Scalar, y: Scalar, theme: &Theme, selected: bool, c: char) {
    renderer.draw_text(&format!("{c}"), [x, y].into(), if selected { theme.cursor_text_style() } else { theme.default_text_style() });
}
