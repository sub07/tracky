pub trait Color {
    fn into_sdl_color(self) -> sdl2::pixels::Color;
}

impl Color for u8 {
    fn into_sdl_color(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(self, self, self)
    }
}

impl Color for (u8, u8, u8) {
    fn into_sdl_color(self) -> sdl2::pixels::Color {
        let (r, g, b) = self;
        sdl2::pixels::Color::RGB(r, g, b)
    }
}

impl Color for (u8, u8, u8, u8) {
    fn into_sdl_color(self) -> sdl2::pixels::Color {
        let (r, g, b, a) = self;
        sdl2::pixels::Color::RGBA(r, g, b, a)
    }
}

impl Color for sdl2::pixels::Color {
    fn into_sdl_color(self) -> sdl2::pixels::Color {
        self
    }
}