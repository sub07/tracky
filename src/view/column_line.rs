use derive_new::new;

use crate::model::ColumnLineElement;
use crate::model::column_line::ColumnLine;
use crate::renderer::Renderer;
use crate::theme::Theme;
use crate::view::Draw;
use crate::view::field::note::NoteFieldDrawData;
use crate::view::field::velocity::VelocityFieldDrawData;

#[derive(new)]
pub struct ColumnLineDrawData {
    is_active_line: bool,
    local_x_cursor: i32,
}

impl Draw for ColumnLine {
    type DrawData = ColumnLineDrawData;

    fn draw(&self, renderer: &mut Renderer, mut x: i32, y: i32, theme: &Theme, ColumnLineDrawData { is_active_line, local_x_cursor }: ColumnLineDrawData) {
        if is_active_line {
            let gray_highlight_width = renderer.glyph_width() * (ColumnLineElement::LINE_LEN + ColumnLineElement::NB_VARIANT - 1) as u32;
            renderer.draw_rect(x, y, gray_highlight_width, renderer.glyph_height(), theme.highlighted_background_color());
        }

        self.note.draw(renderer, x, y, theme, NoteFieldDrawData::new(if is_active_line { Some(local_x_cursor) } else { None }));

        x += renderer.glyph_width() as i32 * (ColumnLineElement::Note.len() + 1) as i32;

        self.velocity.draw(renderer, x, y, theme, VelocityFieldDrawData::new(if is_active_line { Some(local_x_cursor - 3) } else { None }));
    }
}
