use iced::Element;
use iced_lazy::Component;
use iced::{Theme, widget::container};
use iced_native::text;
use iter_tools::Itertools;
use rust_utils_macro::New;

use crate::model::pattern::Column;

use super::column_line::column_line_component;

#[derive(New)]
pub struct ColumnComponent<'a> {
    column: &'a Column,
    cursor_x: Option<i32>,
    cursor_y: i32,
}

pub fn column_component<'a>(column: &'a Column, cursor_x: Option<i32>, cursor_y: i32) -> ColumnComponent<'a> {
    ColumnComponent::new(column, cursor_x, cursor_y)
}

impl<'a, M, R> Component<M, R> for ColumnComponent<'a>
where
    R: text::Renderer<Theme = Theme> + 'static,
{
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<M> {
        None
    }

    fn view(&self, _state: &Self::State) -> iced_native::Element<'_, Self::Event, R> {
        let lines = self
            .column
            .lines
            .iter()
            .enumerate()
            .map(|(line_index, line)| {
                column_line_component(
                    line,
                    self.cursor_x.filter(|_| line_index as i32 == self.cursor_y),
                )
                .into()
            })
            .collect_vec();
        container(iced::widget::Column::with_children(lines)).padding(8).style(iced::theme::Container::Box).into()
    }
}

impl<'a, 'm, Message, Renderer> From<ColumnComponent<'a>> for Element<'m, Message, Renderer>
where
    Message: 'm,
    Renderer: 'static + text::Renderer<Theme = Theme>,
    'a: 'm,
{
    fn from(column: ColumnComponent<'a>) -> Self {
        iced_lazy::component(column)
    }
}
