use rust_utils_macro::New;

use crate::model::pattern::field::Note;
use crate::model::pattern::field::note::NoteField;
use crate::rendering::renderer::Renderer;

use crate::theme::Theme;
use crate::view::Draw;
use crate::view::field::draw_char_input_unit;

#[derive(New)]
pub struct NoteFieldDrawData {
    local_x_selected: Option<i32>,
}

impl Draw for NoteField {
    type DrawData = NoteFieldDrawData;

    fn draw<R: Renderer>(&self, renderer: &mut R, mut x: i32, y: i32, theme: &Theme, NoteFieldDrawData { local_x_selected }: NoteFieldDrawData) {
        let index = match local_x_selected {
            None => -1,
            Some(index) if index == 1 => panic!("Cannot put cursor on not sharp"),
            Some(index) => index,
        };

        let (note, alteration, octave) = match &self.note {
            None => ('.', '.', '.'),
            Some(note_data) => {
                let (note, alteration) = match note_data.note {
                    Note::A => ('A', '-'),
                    Note::B => ('B', '-'),
                    Note::C => ('C', '-'),
                    Note::D => ('D', '-'),
                    Note::E => ('E', '-'),
                    Note::F => ('F', '-'),
                    Note::G => ('G', '-'),
                    Note::CSharp => ('C', '#'),
                    Note::DSharp => ('D', '#'),
                    Note::FSharp => ('F', '#'),
                    Note::GSharp => ('G', '#'),
                    Note::ASharp => ('A', '#'),
                };

                let octave_char = char::from_digit(note_data.octave.value() as u32, 10).unwrap();
                (note, alteration, octave_char)
            }
        };
        draw_char_input_unit(renderer, x, y, theme, index == 0, note);
        x += renderer.glyph_size().w();
        draw_char_input_unit(renderer, x, y, theme, false, alteration);
        x += renderer.glyph_size().w();
        draw_char_input_unit(renderer, x, y, theme, index == 2, octave);
    }
}
