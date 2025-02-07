use joy_macro::New;
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    widgets::{Paragraph, Widget},
};

use crate::model::pattern::{NoteFieldValue, NoteName, PatternLine, PatternLineDescriptor};

use super::theme::THEME;

#[derive(New)]
pub struct PatternLineView<'a> {
    pub line: &'a PatternLine,
    pub is_line_selected: bool,
    pub current_field: Option<i32>,
    pub is_line_played: bool,
}

impl PatternLineView<'_> {
    pub const LINE_WIDTH: u16 =
        PatternLineDescriptor::LINE_LEN as u16 + PatternLineDescriptor::COUNT as u16 - 1;
}

impl Widget for PatternLineView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (note_char_1, note_char_2, octave_char) =
            if let Some(note_value) = self.line.note.value() {
                match note_value {
                    NoteFieldValue::Note(note, octave) => {
                        let (note_1, note_2) = match note {
                            NoteName::A => ('A', '-'),
                            NoteName::B => ('B', '-'),
                            NoteName::C => ('C', '-'),
                            NoteName::D => ('D', '-'),
                            NoteName::E => ('E', '-'),
                            NoteName::F => ('F', '-'),
                            NoteName::G => ('G', '-'),
                            NoteName::CSharp => ('C', '#'),
                            NoteName::DSharp => ('D', '#'),
                            NoteName::FSharp => ('F', '#'),
                            NoteName::GSharp => ('G', '#'),
                            NoteName::ASharp => ('A', '#'),
                        };
                        let octave = octave.value();
                        let octave_char = match octave {
                            0 => '0',
                            1 => '1',
                            2 => '2',
                            3 => '3',
                            4 => '4',
                            5 => '5',
                            6 => '6',
                            7 => '7',
                            8 => '8',
                            9 => '9',
                            _ => panic!("Cannot happen"),
                        };
                        (note_1, note_2, octave_char)
                    }
                    NoteFieldValue::Cut => ('C', 'U', 'T'),
                }
            } else {
                ('.', '.', '.')
            };

        let (vel_char_1, vel_char_2) = if let Some((first, second)) = self.line.velocity.value() {
            (
                char::from_digit(first.value() as u32, 16)
                    .unwrap()
                    .to_ascii_uppercase(),
                char::from_digit(second.value() as u32, 16)
                    .unwrap()
                    .to_ascii_uppercase(),
            )
        } else {
            ('.', '.')
        };

        let (instr_char_1, instr_char_2) =
            if let Some((first, second)) = self.line.instrument.value() {
                (
                    char::from_digit(first.value() as u32, 16)
                        .unwrap()
                        .to_ascii_uppercase(),
                    char::from_digit(second.value() as u32, 16)
                        .unwrap()
                        .to_ascii_uppercase(),
                )
            } else {
                ('.', '.')
            };

        Paragraph::new(format!(
            "{}{}{} {}{} {}{}",
            note_char_1,
            note_char_2,
            octave_char,
            vel_char_1,
            vel_char_2,
            instr_char_1,
            instr_char_2,
        ))
        .style(if self.is_line_played {
            THEME.secondary_cursor
        } else {
            Style::reset()
        })
        .render(area, buf);

        if let Some(current_field) = self.current_field.filter(|_| self.is_line_selected) {
            let offset_x = PatternLineDescriptor::field_index_by_cursor(current_field);
            let cursor_cell = buf
                .cell_mut((area.x + current_field as u16 + offset_x as u16, area.y))
                .unwrap();
            cursor_cell.set_style(THEME.primary_cursor);
        }
    }
}
