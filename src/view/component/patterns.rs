use iced::{
    widget::{text, Column},
    Element,
};
use iced_lazy::Component;
use iced_native::{alignment::Horizontal, widget::scrollable::Properties};
use iter_tools::Itertools;

use crate::model::pattern::PatternCollection;

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
    R: iced_native::text::Renderer<Theme = iced::Theme> + 'static,
{
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<M> {
        None
    }

    fn view(&self, _state: &Self::State) -> iced_native::Element<'_, Self::Event, R> {
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
                    .horizontal_alignment(Horizontal::Right)
                    .size(crate::view::widget::input_unit::DEFAULT_FONT_SIZE)
                    .into()
            })
            .collect_vec();
        let line_number_column = Column::with_children(line_text_numbers).padding([16, 0]);

        iced::widget::scrollable(iced::widget::row![line_number_column, pattern,])
            .id(self.scroll_id.clone())
            .horizontal_scroll(Properties::default())
            .vertical_scroll(Properties::default())
            .into()
    }
}

impl<'a, 'm, Message, Renderer> From<PatternsComponent<'a>> for Element<'m, Message, Renderer>
where
    Message: 'm,
    Renderer: 'static + iced_native::text::Renderer<Theme = iced::Theme>,
    'a: 'm,
{
    fn from(pattern: PatternsComponent<'a>) -> Self {
        iced_lazy::component(pattern)
    }
}
