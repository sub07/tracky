use crate::renderer::WindowRenderer;
use crate::theme::Theme;
use crate::Vec2;

pub mod note;
pub mod velocity;

pub fn draw_char_input_unit<Renderer: WindowRenderer>(renderer: &mut Renderer, pos: Vec2, theme: &Theme, selected: bool, c: char) {
    renderer.draw_text(format!("{c}"), pos, if selected { theme.cursor_text_style() } else { theme.default_text_style() });
}

pub fn draw_input_unit<Renderer: WindowRenderer>(renderer: &mut Renderer, theme: &Theme, c: char, selected: bool, on_selected_line: bool) {
    let mut buffer = [0u8; 4];
    let text = c.encode_utf8(&mut buffer);
    let theme = match (selected, on_selected_line) {
        (_, false) => theme.pattern_text_style(),
        (true, true) => theme.pattern_selected_unit_text_style(),
        (false, true) => theme.pattern_selected_line_text_style(),
    };
    renderer.draw_text(text, Vec2::ZERO, theme);
}
