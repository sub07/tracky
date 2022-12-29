use derive_new::new;

use crate::model::pattern_line::PatternLine;
use crate::model::{PatternLineElement};
use crate::renderer::Renderer;
use crate::view::Draw;
use crate::view::field::note::NoteFieldDrawData;
use crate::view::field::velocity::VelocityFieldDrawData;

#[derive(new)]
pub struct PatternLineDrawData {
    is_active_line: bool,
    local_x_cursor: i32,
}

impl Draw for PatternLine {
    type DrawData = PatternLineDrawData;

    fn draw(&self, renderer: &mut Renderer, mut x: i32, y: i32, PatternLineDrawData { is_active_line, local_x_cursor }: PatternLineDrawData) {
        if is_active_line {
            let gray_highlight_width = renderer.glyph_width() * (PatternLineElement::LINE_LEN + PatternLineElement::NB_VARIANT - 1) as u32;
            renderer.draw_rect(x, y, gray_highlight_width, renderer.glyph_height(), (100, 100, 120));
        }

        self.note.draw(renderer, x, y, NoteFieldDrawData::new(if is_active_line { Some(local_x_cursor) } else { None }));

        x += renderer.glyph_width() as i32 * (PatternLineElement::Note.len() + 1) as i32;

        self.velocity.draw(renderer, x, y, VelocityFieldDrawData::new(if is_active_line { Some(local_x_cursor - 3) } else { None }));
    }
}
