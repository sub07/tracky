use iced::{
    widget::{container, Row},
    Element,
};
use iced_lazy::Component;
use iced_native::{
    text,
    widget::{scrollable::Properties, Operation},
    Theme,
};
use iter_tools::Itertools;
use rust_utils_macro::New;

use crate::model::pattern::{ColumnLineElement, Pattern};

use super::column::column_component;

#[derive(New)]
pub struct PatternComponent<'a> {
    pattern: &'a Pattern,
    cursor_x: i32,
    cursor_y: i32,
}

pub fn pattern_component<'a>(
    pattern: &'a Pattern,
    cursor_x: i32,
    cursor_y: i32,
) -> PatternComponent<'a> {
    PatternComponent::new(pattern, cursor_x, cursor_y)
}

impl<'a, M, R> Component<M, R> for PatternComponent<'a>
where
    R: text::Renderer<Theme = Theme> + 'static,
{
    type State = ();

    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<M> {
        None
    }

    fn view(&self, _state: &Self::State) -> iced_native::Element<'_, Self::Event, R> {
        let cursor_column_index = self.cursor_x / ColumnLineElement::LINE_LEN;
        let cursor_column_local = self.cursor_x % ColumnLineElement::LINE_LEN;
        let columns = self
            .pattern
            .columns
            .iter()
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

impl<'a, 'm, Message, Renderer> From<PatternComponent<'a>> for Element<'m, Message, Renderer>
where
    Message: 'm,
    Renderer: 'static + text::Renderer<Theme = Theme>,
    'a: 'm,
{
    fn from(pattern: PatternComponent<'a>) -> Self {
        iced_lazy::component(pattern)
    }
}
