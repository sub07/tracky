use log::warn;
use ratatui::{
    prelude::{Buffer, Rect},
    symbols,
    widgets::Widget,
};

use crate::{assert_log_bail, view::theme::THEME};

pub struct Slider {
    min: f32,
    // Inclusive
    max: f32,
    value: f32,
}

impl Slider {
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        Slider { min, max, value }
    }
}

impl Widget for Slider {
    fn render(self, area: Rect, buf: &mut Buffer) {
        assert_log_bail!(
            self.min <= self.max,
            "Slider with larger minimum than maximum is not possible"
        );
        let value = self.value.clamp(self.min, self.max);
        if self.value != value {
            warn!(
                "Slider value {} is not in min / max bounds ([{}; {}]). Clamped to {}",
                self.value, self.min, self.max, value
            );
        }

        let ratio = (value - self.min) / (self.max - self.min);

        let filled_width = area.width as f32 * ratio;
        let end = area.left() + filled_width.floor() as u16;
        for y in area.top()..area.bottom() {
            for x in area.left()..end {
                buf[(x, y)]
                    .set_symbol(symbols::block::FULL)
                    .set_style(THEME.normal);
            }
            if ratio < 1.0 {
                fn get_unicode_block<'a>(frac: f32) -> &'a str {
                    match (frac * 8.0).round() as u16 {
                        1 => symbols::block::ONE_EIGHTH,
                        2 => symbols::block::ONE_QUARTER,
                        3 => symbols::block::THREE_EIGHTHS,
                        4 => symbols::block::HALF,
                        5 => symbols::block::FIVE_EIGHTHS,
                        6 => symbols::block::THREE_QUARTERS,
                        7 => symbols::block::SEVEN_EIGHTHS,
                        8 => symbols::block::FULL,
                        _ => " ",
                    }
                }

                buf[(end, y)].set_symbol(get_unicode_block(filled_width.fract()));
            }
        }
    }
}
