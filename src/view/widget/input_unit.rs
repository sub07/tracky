use iced::alignment::Vertical;
use iced::{Background, Color, Element, Length, Point, Rectangle, Size, Theme};
use iced_native::alignment::Horizontal;
use iced_native::layout::{Limits, Node};
use iced_native::renderer::{BorderRadius, Quad, Style};
use iced_native::widget::Tree;
use iced_native::{text, Layout, Widget};

pub struct InputUnitWidget {
    pub value: Option<char>,
    pub selected: bool,
}

impl InputUnitWidget {
    fn value(&self) -> char {
        self.value.unwrap_or('.')
    }
}

const DEFAULT_SIZE: f32 = 30.0;

impl<Message, Renderer> Widget<Message, Renderer> for InputUnitWidget
where
    Renderer: text::Renderer<Theme = Theme>,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let mut str_buf = [0; 4];
        let (w, h) = renderer.measure(
            self.value().encode_utf8(&mut str_buf),
            DEFAULT_SIZE,
            Renderer::Font::default(),
            limits.max(),
        );
        Node::new(Size::new(w, h))
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &Style,
        layout: Layout<'_>,
        _cursor_position: Point,
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
        let text = text::Text {
            font: Renderer::Font::default(),
            size: DEFAULT_SIZE,
            bounds: layout.bounds(),
            content: self.value().encode_utf8(&mut str_buf),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Top,
            color: text_color,
        };
        renderer.fill_text(text);
    }
}

impl<'a, Message, Renderer> From<InputUnitWidget> for Element<'a, Message, Renderer>
where
    Renderer: text::Renderer<Theme = Theme>,
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
