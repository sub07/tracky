use iced::{
    widget::{component, row, Component},
    Element,
};
use rust_utils_macro::New;

use crate::{
    model::{
        field::{NoteFieldValue, NoteName},
        pattern::PatternLine,
    },
    view::{
        widget::input_unit::{input_unit, input_unit_spacer},
        CustomRenderer,
    },
};

#[derive(New)]
pub struct ColumnLineComponent<'a> {
    line: &'a PatternLine,
    cursor_x: Option<i32>,
}

pub fn column_line_component(
    model: &PatternLine,
    cursor_x: Option<i32>,
) -> ColumnLineComponent<'_> {
    ColumnLineComponent::new(model, cursor_x)
}

impl<'a, M, R> Component<M, R> for ColumnLineComponent<'a>
where
    R: CustomRenderer + 'static,
{
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<M> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, R> {
        let (note_char_1, note_char_2, octave_char) =
            if let Some(note_value) = self.line.note.value() {
                match note_value {
                    NoteFieldValue::Note((note, octave)) => {
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
                        (Some(note_1), Some(note_2), Some(octave_char))
                    }
                    NoteFieldValue::Cut => (Some('C'), Some('U'), Some('T')),
                }
            } else {
                (None, None, None)
            };

        let (vel_char_1, vel_char_2) = if let Some((first, second)) = self.line.velocity.value() {
            (
                Some(
                    char::from_digit(first.value() as u32, 16)
                        .unwrap()
                        .to_ascii_uppercase(),
                ),
                Some(
                    char::from_digit(second.value() as u32, 16)
                        .unwrap()
                        .to_ascii_uppercase(),
                ),
            )
        } else {
            (None, None)
        };

        let (instr_char_1, instr_char_2) =
            if let Some((first, second)) = self.line.instrument.value() {
                (
                    Some(
                        char::from_digit(first.value() as u32, 16)
                            .unwrap()
                            .to_ascii_uppercase(),
                    ),
                    Some(
                        char::from_digit(second.value() as u32, 16)
                            .unwrap()
                            .to_ascii_uppercase(),
                    ),
                )
            } else {
                (None, None)
            };

        row![
            input_unit(note_char_1, self.cursor_x.is_some_and(|x| x == 0)),
            input_unit(note_char_2, false),
            input_unit(octave_char, self.cursor_x.is_some_and(|x| x == 2)),
            input_unit_spacer(),
            input_unit(vel_char_1, self.cursor_x.is_some_and(|x| x == 3)),
            input_unit(vel_char_2, self.cursor_x.is_some_and(|x| x == 4)),
            input_unit_spacer(),
            input_unit(instr_char_1, self.cursor_x.is_some_and(|x| x == 5)),
            input_unit(instr_char_2, self.cursor_x.is_some_and(|x| x == 6)),
        ]
        .into()
    }
}

impl<'a, 'm, M, R> From<ColumnLineComponent<'a>> for Element<'m, M, R>
where
    M: 'm,
    R: 'static + CustomRenderer,
    'a: 'm,
{
    fn from(column_line: ColumnLineComponent<'a>) -> Self {
        component(column_line)
    }
}
