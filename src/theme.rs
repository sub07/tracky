macro_rules! declare_theme_struct {
    ($($field_name:ident,)*) => {
        #[derive(Debug, Clone, Copy)]
        pub struct Theme {
            $(
                $field_name: sdl2::pixels::Color,
            )*
        }

        impl Theme {
            pub fn new(
                $(
                    $field_name: impl crate::color::Color,
                )*
            ) -> Theme {
                Theme {
                    $(
                        $field_name: $field_name.into_sdl_color(),
                    )*
                }
            }

            $(
                pub fn $field_name(&self) -> sdl2::pixels::Color {
                    self.$field_name
                }
            )*
        }
    };
}

declare_theme_struct!(
    text_color,
    selected_text_color,
    pattern_background_color,
    selected_background_color,
    highlighted_background_color,
);

impl Theme {
    pub fn default_dark() -> Theme {
        Theme::new(
            255,
            0,
            (40, 40, 60),
            255,
            (100, 100, 120),
        )
    }
}