use derive_new::new;

use crate::model::pattern_line::PatternLine;
use crate::renderer::Renderer;
use crate::view::Draw;
use crate::view::field::note::NoteFieldDrawData;
use crate::view::field::velocity::VelocityFieldDrawData;

#[derive(new)]
pub struct PatternLineDrawData {
    is_active_line: bool,
    local_x_cursor: Option<usize>,
}

impl Draw for PatternLine {
    type DrawData = PatternLineDrawData;

    fn draw(&self, renderer: &mut Renderer, mut x: i32, y: i32, PatternLineDrawData { is_active_line, local_x_cursor }: PatternLineDrawData) {
        if let Some(index) = local_x_cursor && index > 3 {
            panic!("Invalid local index: {x}");
        }

        if is_active_line {
            let gray_highlight_width = renderer.glyph_width() * 6 as u32;
            renderer.draw_rect(x, y, gray_highlight_width, renderer.glyph_height(), (100, 100, 120));
        }
        let local_x_note_cursor = match local_x_cursor {
            Some(x) if x <= 1 && is_active_line => Some(x),
            _ => None,
        };

        self.note.draw(renderer, x, y, NoteFieldDrawData::new(local_x_note_cursor));

        x += renderer.glyph_width() as i32 * 4;

        let local_x_velocity_cursor = match local_x_cursor {
            Some(x) if x >= 2 && x <= 3 && is_active_line => Some(x - 2),
            _ => None,
        };

        self.velocity.draw(renderer, x, y, VelocityFieldDrawData::new(local_x_velocity_cursor));
    }
}
