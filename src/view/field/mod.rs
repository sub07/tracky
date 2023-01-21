use crate::renderer::Renderer;
use crate::theme::Theme;

pub mod note;
pub mod velocity;

pub fn draw_char_input_unit(renderer: &mut Renderer, x: i32, y: i32, theme: &Theme, selected: bool, c: char) {
    renderer.draw_text(
        format!("{c}"),
        x, y,
        if selected { theme.cursor_text_style() } else { theme.default_text_style() },
    );
}
