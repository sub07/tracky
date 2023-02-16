use rust_utils_macro::New;

use crate::model::pattern::column_line::ColumnLine;
use crate::model::pattern::ColumnLineElement;
use crate::renderer::WindowRenderer;
use crate::theme::Theme;
use crate::Vec2;
use crate::view::Draw;
use crate::view::field::note::NoteFieldDrawData;
use crate::view::field::velocity::VelocityFieldDrawData;

#[derive(New)]
pub struct ColumnLineDrawData {
    is_active_line: bool,
    local_x_cursor: i32,
}

impl Draw for ColumnLine {
    type DrawData = ColumnLineDrawData;

    fn draw<Renderer: WindowRenderer>(&self, renderer: &mut Renderer, mut x: i32, y: i32, theme: &Theme, ColumnLineDrawData { is_active_line, local_x_cursor }: ColumnLineDrawData) {
        if is_active_line {
            let gray_highlight_width = renderer.glyph_width() * (ColumnLineElement::LINE_LEN + ColumnLineElement::SIZE as i32 - 1);
            renderer.draw_rect(Vec2::new(x, y), Vec2::new(gray_highlight_width, renderer.glyph_height()), theme.selected_line_background_color, true);
        }

        self.note_field.draw(renderer, x, y, theme, NoteFieldDrawData::new(if is_active_line { Some(local_x_cursor) } else { None }));

        x += renderer.glyph_width() * (ColumnLineElement::Note.len() + 1) as i32;

        self.velocity_field.draw(renderer, x, y, theme, VelocityFieldDrawData::new(if is_active_line { Some(local_x_cursor - 3) } else { None }));
    }
}
