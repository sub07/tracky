use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer::{Quad, Style},
        widget::Tree,
        Layout, Text, Widget,
    },
    alignment::{Horizontal, Vertical},
    mouse::Cursor,
    widget::text::{LineHeight, Shaping},
    Background, BorderRadius, Color, Element, Length, Rectangle, Size,
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

pub const DEFAULT_FONT_SIZE: f32 = 18.0;

impl<M, R> Widget<M, R> for InputUnitWidget
where
    R: CustomRenderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &R, limits: &Limits) -> Node {
        let mut str_buf = [0; 4];
        let Size { width, height } = renderer.measure(
            self.value().encode_utf8(&mut str_buf),
            DEFAULT_FONT_SIZE,
            LineHeight::default(),
            MONOSPACED_FONT,
            limits.max(),
            Shaping::default(),
        );

        Node::new(Size::new(width, height))
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut R,
        theme: &R::Theme,
        _style: &Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
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
                    border_color: Color::TRANSPARENT,
                    border_width: 0.0,
                    border_radius: BorderRadius::from(0.0),
                },
                Background::from(background_color),
            );
        }

        let mut str_buf = [0; 4];
        let text = Text {
            font: MONOSPACED_FONT,
            size: DEFAULT_FONT_SIZE,
            bounds: layout.bounds(),
            content: self.value().encode_utf8(&mut str_buf),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Top,
            color: text_color,
            line_height: Default::default(),
            shaping: Default::default(),
        };
        renderer.fill_text(text);
    }
}

impl<'a, M, R> From<InputUnitWidget> for Element<'a, M, R>
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
