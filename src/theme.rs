use rust_utils_macro::{Getter, New};

use crate::mono_font_atlas::{TextAlignment, TextStyle};

#[derive(New, Getter)]
pub struct Theme {
    default_text_color: (u8, u8, u8),
    selected_text_color: (u8, u8, u8),
    pattern_background_color: (u8, u8, u8),
    selected_line_background_color: (u8, u8, u8),
}

impl Theme {
    pub fn default_dark() -> Theme {
        Theme::new(
            (255, 255, 255),
            (0, 0, 0),
            (40, 40, 60),
            (100, 100, 120),
        )
    }

    pub fn selected_line_count_style(&self) -> TextStyle {
        TextStyle::new(
            self.default_text_color(),
            None,
            TextAlignment::Right,
        )
    }

    pub fn line_count_style(&self) -> TextStyle {
        TextStyle::new(
            self.default_text_color(),
            None,
            TextAlignment::Right,
        )
    }

    pub fn column_number_style(&self) -> TextStyle {
        TextStyle::new(
            self.default_text_color(),
            Some(self.pattern_background_color()),
            TextAlignment::Left,
        )
    }

    pub fn default_text_style(&self) -> TextStyle {
        TextStyle::new(
            self.default_text_color(),
            None,
            TextAlignment::Left,
        )
    }

    pub fn cursor_text_style(&self) -> TextStyle {
        TextStyle::new(
            self.selected_text_color(),
            Some(self.default_text_color()),
            TextAlignment::Left,
        )
    }

    pub fn pattern_index_style(&self) -> TextStyle {
        self.column_number_style()
    }
}