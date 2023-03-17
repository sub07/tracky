use rust_utils_macro::New;

use crate::rendering::font::{TextAlignment, TextStyle};

#[derive(New)]
pub struct Theme {
    pub background_color: (u8, u8, u8),
    pub default_text_color: (u8, u8, u8),
    pub selected_text_color: (u8, u8, u8),
    pub pattern_background_color: (u8, u8, u8),
    pub selected_line_background_color: (u8, u8, u8),
    #[cfg(debug_assertions)] pub debug_bound_color: (u8, u8, u8),
}

macro_rules! define_text_style {
    ($alias:ident = $name:ident) => { // Alias entry point
        define_text_style!($alias, {|s: &Theme| s.$name()});
    };
    ($name:ident = fg: $fg_color:ident, bg: $bg_color:ident, align: $align:ident) => { // with bg entry point
        define_text_style!($name, $fg_color, {|s: &Theme| Some(s.$bg_color)}, $align);
    };
    ($name:ident = fg: $fg_color:ident, align: $align:ident) => { // without bg entry point
        define_text_style!($name, $fg_color, {|_: &Theme| None}, $align);
    };
    ($name:ident, $fg_color:ident, $bg_color:block, $align:ident) => { // Merge fg & bg colors
        define_text_style!($name, {|s: &Theme| s.$fg_color}, $bg_color, {TextAlignment::$align});
    };
    ($name:ident, $fg_color:block, $bg_color:block, $align:block) => { // Create method body
        define_text_style!($name, {|s: &Theme| TextStyle::new(($fg_color)(s), ($bg_color)(s), $align)});
    };
    ($fn_name:ident, $fn_body: block) => { // Emit method
        pub fn $fn_name(&self) -> TextStyle {($fn_body)(self)}
    };
}

impl Theme {
    pub fn default_dark() -> Theme {
        Theme::new(
            (20, 20, 20),
            (255, 255, 255),
            (0, 0, 0),
            (40, 40, 60),
            (100, 100, 120),
            #[cfg(debug_assertions)] (10, 255, 30),
        )
    }

    define_text_style!(pattern_text_style = fg: default_text_color, bg: pattern_background_color, align: Left);
    define_text_style!(pattern_selected_line_text_style = fg: default_text_color, bg: selected_line_background_color, align: Left);
    define_text_style!(pattern_selected_unit_text_style = fg: selected_text_color, bg: default_text_color, align: Left);

    define_text_style!(line_count_style = fg: default_text_color, align: Right);
    define_text_style!(default_text_style = fg: default_text_color, align: Left);
    define_text_style!(column_number_style = fg: default_text_color, bg: pattern_background_color, align: Left);
    define_text_style!(cursor_text_style = fg: selected_text_color, bg: default_text_color, align: Left);
    define_text_style!(pattern_index_style = column_number_style);
}
