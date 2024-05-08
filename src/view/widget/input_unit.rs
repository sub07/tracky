use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer::{Quad, Style},
        text::Paragraph,
        widget::Tree,
        Layout, Text, Widget,
    },
    alignment::{Horizontal, Vertical},
    border::Radius,
    mouse::Cursor,
    widget::text::{LineHeight, Shaping},
    Background, Border, Color, Element, Length, Pixels, Point, Rectangle, Shadow, Size, Theme,
};

use crate::view::{CustomRenderer, MONOSPACED_FONT};

pub struct InputUnitWidget {
    pub value: Option<char>,
    pub selected: bool,
}

impl InputUnitWidget {
    fn value(&self) -> char {
        self.value.unwrap_or('.')
    }
}

pub const DEFAULT_FONT_SIZE: Pixels = Pixels(18.0);

fn get_paragraph(content: &str, bounds: Size) -> iced_graphics::text::Paragraph {
    iced_graphics::text::Paragraph::with_text(Text {
        content,
        bounds,
        size: DEFAULT_FONT_SIZE,
        line_height: LineHeight::default(),
        font: MONOSPACED_FONT,
        horizontal_alignment: Horizontal::Left,
        vertical_alignment: Vertical::Top,
        shaping: Shaping::default(),
    })
}

impl<M, R> Widget<M, iced::Theme, R> for InputUnitWidget
where
    R: CustomRenderer,
{
    fn layout(&self, _: &mut Tree, _: &R, limits: &Limits) -> Node {
        Node::new(get_paragraph(self.value().encode_utf8(&mut [0; 4]), limits.max()).min_bounds())
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut R,
        theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let (text_color, background_color) = if self.selected {
            (theme.palette().background, theme.palette().text)
        } else {
            (theme.palette().text, theme.palette().background)
        };
        if self.selected {
            renderer.fill_quad(
                Quad {
                    bounds: layout.bounds(),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::from(0.0),
                    },
                    shadow: Shadow::default(),
                },
                Background::from(background_color),
            );
        }
        renderer.fill_paragraph(
            &get_paragraph(
                self.value().encode_utf8(&mut [0; 4]),
                layout.bounds().size(),
            ),
            Point::ORIGIN,
            text_color,
            *viewport,
        );
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }
}

impl<'a, M, R> From<InputUnitWidget> for Element<'a, M, Theme, R>
where
    R: CustomRenderer,
{
    fn from(v: InputUnitWidget) -> Self {
        Self::new(v)
    }
}

pub fn input_unit(value: Option<char>, selected: bool) -> InputUnitWidget {
    InputUnitWidget { value, selected }
}

pub fn input_unit_spacer() -> InputUnitWidget {
    InputUnitWidget {
        value: Some(' '),
        selected: false,
    }
}
