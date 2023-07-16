use iced::{widget::Row, Element};
use iced_lazy::Component;
use iced_native::{text, Theme};
use iter_tools::Itertools;
use rust_utils_macro::New;

use crate::model::pattern::{ColumnLineElement, Pattern};

use super::column::column_component;

#[derive(New)]
pub struct PatternComponent {
    pattern: Pattern,
    cursor_x: i32,
    cursor_y: i32,
}

pub fn pattern_component(pattern: Pattern, cursor_x: i32, cursor_y: i32) -> PatternComponent {
    PatternComponent::new(pattern, cursor_x, cursor_y)
}

impl<M, R> Component<M, R> for PatternComponent
where
    R: text::Renderer<Theme = Theme> + 'static,
{
    type State = ();

    type Event = ();

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<M> {
        None
    }

    fn view(&self, state: &Self::State) -> iced_native::Element<'_, Self::Event, R> {
        let columns = self
            .pattern
            .columns
            .iter()
            .enumerate()
            .map(|(column_index, column)| {
                let cursor_column_index = self.cursor_x / ColumnLineElement::LINE_LEN;
                let cursor_column_local = self.cursor_x % ColumnLineElement::LINE_LEN;
                let cursor_x = if column_index as i32 == cursor_column_index {
                    Some(cursor_column_local)
                } else {
                    None
                };
                column_component(column.clone(), cursor_x, self.cursor_y).into()
            })
            .collect_vec();
        Row::with_children(columns).spacing(8).into()
    }
}

impl<'a, Message, Renderer> From<PatternComponent> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'static + text::Renderer<Theme = Theme>,
{
    fn from(pattern: PatternComponent) -> Self {
        iced_lazy::component(pattern)
    }
}
