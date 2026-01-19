use ratatui::style::{Color, Style};

pub const THEME: Theme = Theme {
    normal: Style::new()
        .bg(Color::Rgb(27, 25, 42))
        .fg(Color::Rgb(201, 198, 219)),

    elevated_1: Style::new()
        .bg(Color::Rgb(42, 45, 62))
        .fg(Color::Rgb(235, 205, 253)),

    elevated_2: Style::new()
        .bg(Color::Rgb(62, 65, 85))
        .fg(Color::Rgb(242, 242, 238)),

    primary_cursor: Style::new()
        .bg(Color::Rgb(0, 148, 198))
        .fg(Color::Rgb(3, 3, 3)),

    secondary_cursor: Style::new()
        .bg(Color::Rgb(100, 100, 100))
        .fg(Color::Rgb(230, 230, 230)),

    primary: Color::Rgb(0, 148, 198),
    success: Color::Rgb(3, 252, 124),
    secondary: Color::Rgb(100, 100, 100),
    danger: Color::Rgb(253, 68, 4),
};

pub struct Theme {
    pub normal: Style,
    pub elevated_1: Style,
    pub elevated_2: Style,
    pub primary_cursor: Style,
    pub secondary_cursor: Style,

    pub primary: Color,
    pub success: Color,
    pub secondary: Color,
    pub danger: Color,
}
