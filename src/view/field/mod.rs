use crate::mono_font_atlas::TextAlignment;
use crate::renderer::Renderer;
use crate::theme::Theme;

pub mod note;
pub mod velocity;

pub fn draw_char_input_unit(renderer: &mut Renderer, x: i32, y: i32, theme: &Theme, selected: bool, c: char) {
    if selected {
        renderer.draw_text_with_background(format!("{c}"), x, y, theme.selected_text_color(), theme.selected_background_color(), TextAlignment::Left);
    } else {
        renderer.draw_text(format!("{c}"), x, y, theme.text_color(), TextAlignment::Left)
    }
}
