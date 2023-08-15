use iced::{
    alignment::Horizontal,
    widget::{
        component,
        scrollable::{Direction, Properties},
        text, Component,
    },
    Element,
};
use iter_tools::Itertools;

use crate::{model::pattern::PatternCollection, view::CustomRenderer, MONOSPACED_FONT};

use super::pattern::pattern_component;

pub struct PatternsComponent<'a> {
    pattern_collection: &'a PatternCollection,
    scroll_id: iced::widget::scrollable::Id,
}

pub fn patterns_component<'a>(
    pattern_collection: &'a PatternCollection,
    scroll_id: iced::widget::scrollable::Id,
) -> PatternsComponent<'a> {
    PatternsComponent {
        pattern_collection,
        scroll_id,
    }
}

impl<'a, M, R> Component<M, R> for PatternsComponent<'a>
where
    R: CustomRenderer + 'static,
{
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<M> {
        None
    }

    fn view(&self, _state: &Self::State) -> iced::Element<'_, Self::Event, R> {
        let pattern = pattern_component(
            self.pattern_collection.current_pattern(),
            self.pattern_collection.cursor_x,
            self.pattern_collection.cursor_y,
        );
        let line_text_numbers = (0..self.pattern_collection.current_pattern().columns[0]
            .lines
            .len())
            .map(|line_index| {
                text(format!("{: >3}", line_index))
                    .font(MONOSPACED_FONT)
                    .horizontal_alignment(Horizontal::Right)
                    .size(crate::view::widget::input_unit::DEFAULT_FONT_SIZE)
                    .into()
            })
            .collect_vec();
        let line_number_column =
            iced::widget::Column::with_children(line_text_numbers).padding([16, 0]);

        iced::widget::scrollable(iced::widget::row![line_number_column, pattern,])
            .id(self.scroll_id.clone())
            .direction(Direction::Both {
                vertical: Properties::default(),
                horizontal: Properties::default(),
            })
            .into()
    }
}

impl<'a, 'm, M, R> From<PatternsComponent<'a>> for Element<'m, M, R>
where
    M: 'm,
    R: 'static + CustomRenderer,
    'a: 'm,
{
    fn from(pattern: PatternsComponent<'a>) -> Self {
        component(pattern)
    }
}
