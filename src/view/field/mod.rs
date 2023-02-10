use crate::renderer::{WindowRenderer};
use crate::theme::Theme;
use crate::Vec2;

pub mod note;
pub mod velocity;

pub fn draw_char_input_unit<Renderer: WindowRenderer>(renderer: &mut Renderer, x: i32, y: i32, theme: &Theme, selected: bool, c: char) {
    renderer.draw_text(
        format!("{c}"),
        Vec2::new(x, y),
        if selected { theme.cursor_text_style() } else { theme.default_text_style() },
    );
}
