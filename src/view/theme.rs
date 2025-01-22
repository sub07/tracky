use ratatui::style::Color;

pub const THEME: Theme = Theme {
    background: Color::Rgb(27, 25, 42), // Dark navy with slight variation
    elevated_background: Color::Rgb(42, 45, 62), // Subtle blue-gray with a hint of green
    elevated_background_2: Color::Rgb(62, 65, 85), // Slightly brighter blue-gray for contrast
    cursor_background: Color::Rgb(0, 148, 198), // Vibrant cyan for cursor

    on_background: Color::Rgb(201, 198, 219), // Light blue-gray for text
    on_elevated_background: Color::Rgb(235, 205, 253), // Soft lavender with slight variation
    on_elevated_background_2: Color::Rgb(242, 242, 238), // Slightly off-white for high contrast text
    on_cursor: Color::Rgb(3, 3, 3),                      // Nearly black for contrast on the cursor

    success: Color::Rgb(3, 252, 124), // Bright sea green for success
    danger: Color::Rgb(253, 68, 4),   // Vibrant orange-red for dangerbrant orange-red for danger
};

pub struct Theme {
    pub background: Color,
    pub elevated_background: Color,
    pub elevated_background_2: Color,
    pub cursor_background: Color,

    pub on_background: Color,
    pub on_elevated_background: Color,
    pub on_elevated_background_2: Color,
    pub on_cursor: Color,

    pub success: Color,
    pub danger: Color,
}
