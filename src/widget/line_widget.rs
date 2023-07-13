use iced::widget::Row;
use iced::{Font, Length, Point, Rectangle, Size, Theme};
use iced_native::layout::{Limits, Node};
use iced_native::renderer::Style;
use iced_native::widget::Tree;
use iced_native::{text, Layout, Widget};
use iter_tools::Itertools;

use crate::widget::input_unit::InputUnitWidget;

pub struct LineWidget {
    inputs_unit: [InputUnitWidget; 5],
}

impl<M, R> Widget<M, R> for LineWidget
where
    R: text::Renderer<Font = Font, Theme = Theme>,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &R, limits: &Limits) -> Node {
        Row::with_children(
            self.inputs_unit
                .map(|input_unit| input_unit.into())
                .collect_vec(),
        )
        .layout(renderer, limits)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut R,
        theme: &R::Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        todo!()
    }
}
