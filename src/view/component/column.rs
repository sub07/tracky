use iced::{
    widget::{component, container, Component},
    Element, Theme,
};
use iter_tools::Itertools;
use rust_utils_macro::New;

use crate::{model::pattern::ColumnView, view::CustomRenderer};

use super::column_line::column_line_component;

#[derive(New)]
pub struct ColumnComponent<'a> {
    column: ColumnView<'a>,
    cursor_x: Option<i32>,
    cursor_y: i32,
}

pub fn column_component(
    column: ColumnView<'_>,
    cursor_x: Option<i32>,
    cursor_y: i32,
) -> ColumnComponent<'_> {
    ColumnComponent::new(column, cursor_x, cursor_y)
}

impl<'a, M, R> Component<M, Theme, R> for ColumnComponent<'a>
where
    R: CustomRenderer + 'static,
{
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<M> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Theme, R> {
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
        container(
            container(iced::widget::Column::with_children(lines))
                .padding(8)
                .style(iced::theme::Container::Box),
        )
        .padding(4)
        .into()
    }
}

impl<'a, 'm, M, R> From<ColumnComponent<'a>> for Element<'m, M, Theme, R>
where
    M: 'm,
    R: 'static + CustomRenderer,
    'a: 'm,
{
    fn from(column: ColumnComponent<'a>) -> Self {
        component(column)
    }
}
