use iced::{
    widget::{component, container, Component, Row},
    Element,
};
use iter_tools::Itertools;
use rust_utils_macro::New;

use crate::{
    model::pattern::{self, PatternView},
    view::CustomRenderer,
};

use super::column::column_component;

#[derive(New)]
pub struct PatternComponent<'a> {
    pattern: PatternView<'a>,
    cursor_x: i32,
    cursor_y: i32,
}

pub fn pattern_component<'a>(
    pattern: PatternView<'a>,
    cursor_x: i32,
    cursor_y: i32,
) -> PatternComponent<'a> {
    PatternComponent::new(pattern, cursor_x, cursor_y)
}

impl<'a, M, R> Component<M, R> for PatternComponent<'a>
where
    R: CustomRenderer + 'static,
{
    type State = ();

    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<M> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, R> {
        let cursor_column_index = self.cursor_x / pattern::LineField::LINE_LEN;
        let cursor_column_local = self.cursor_x % pattern::LineField::LINE_LEN;
        let columns = self
            .pattern
            .columns()
            .enumerate()
            .map(|(column_index, column)| {
                let cursor_x = if column_index as i32 == cursor_column_index {
                    Some(cursor_column_local)
                } else {
                    None
                };
                column_component(column, cursor_x, self.cursor_y).into()
            })
            .collect_vec();
        container(Row::with_children(columns)).padding(4).into()
    }
}

impl<'a, 'm, M, R> From<PatternComponent<'a>> for Element<'m, M, R>
where
    M: 'm,
    R: 'static + CustomRenderer,
    'a: 'm,
{
    fn from(pattern: PatternComponent<'a>) -> Self {
        component(pattern)
    }
}
