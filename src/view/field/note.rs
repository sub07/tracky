use derive_new::new;

use crate::model::field::note::NoteField;
use crate::renderer::Renderer;
use crate::view::Draw;

#[derive(new)]
pub struct NoteFieldDrawData {
    local_x_selected: Option<i32>,
}

impl Draw for NoteField {
    type DrawData = NoteFieldDrawData;

    fn draw(&self, renderer: &mut Renderer, mut x: i32, y: i32, NoteFieldDrawData { local_x_selected }: NoteFieldDrawData) {
        let index = match local_x_selected {
            None => -1,
            Some(index) => index,
        };

        self.note1.draw(renderer, x, y, index == 0 || index == 1);
        x += renderer.glyph_width() as i32;
        self.note2.draw(renderer, x, y, index == 0 || index == 1);
        x += renderer.glyph_width() as i32;
        self.octave.draw(renderer, x, y, index == 2);
    }
}
