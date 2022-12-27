use derive_new::new;

use crate::model::field::note::NoteField;
use crate::renderer::Renderer;
use crate::view::Draw;

#[derive(new)]
pub struct NoteFieldDrawData {
    local_x_selected: Option<usize>,
}

impl Draw for NoteField {
    type DrawData = NoteFieldDrawData;

    fn draw(&self, renderer: &mut Renderer, mut x: i32, y: i32, NoteFieldDrawData { local_x_selected }: NoteFieldDrawData) {
        let index = match local_x_selected {
            Some(x) if x <= 1 => x,
            None => usize::MAX,
            Some(x) => panic!("Invalid local index: {x}"),
        };
        self.note1.draw(renderer, x, y, index == 0);
        x += renderer.glyph_width() as i32;
        self.note2.draw(renderer, x, y, index == 0);
        x += renderer.glyph_width() as i32;
        self.octave.draw(renderer, x, y, index == 1);
    }
}
