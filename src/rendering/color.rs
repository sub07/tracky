use tiny_skia::Color;

pub trait IntoColor {
    fn into_color(self) -> Color;
}

impl IntoColor for (u8, u8, u8) {
    fn into_color(self) -> Color {
        let (r, g, b) = self;
        Color::from_rgba8(r, g, b, 255)
    }
}

impl IntoColor for (u8, u8, u8, u8) {
    fn into_color(self) -> Color {
        let (r, g, b, a) = self;
        Color::from_rgba8(r, g, b, a)
    }
}
