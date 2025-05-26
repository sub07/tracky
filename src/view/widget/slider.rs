use log::warn;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    symbols,
    text::ToLine,
    widgets::{Gauge, LineGauge, Widget},
};

use crate::{assert_log_bail, view::theme::THEME};

pub struct Slider {
    min: i32,
    // Inclusive
    max: i32,
    value: i32,
}

impl Slider {
    pub fn new(min: i32, max: i32, value: i32) -> Self {
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
        let value_str = value.to_string();
        if self.value != value {
            warn!(
                "Slider value {} is not in min / max bounds ([{}; {}]). Clamped to {}",
                self.value, self.min, self.max, value
            );
        }

        let ratio = (value - self.min) as f64 / (self.max - self.min) as f64;

        let [bar_area, value_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(self.max.to_string().len() as u16),
        ])
        .spacing(1)
        .areas(area);

        value_str.to_line().right_aligned().render(value_area, buf);

        let filled_width = f64::from(bar_area.width) * ratio;
        let end = bar_area.left() + filled_width.floor() as u16;
        for y in bar_area.top()..bar_area.bottom() {
            for x in bar_area.left()..end {
                buf[(x, y)]
                    .set_symbol(symbols::block::FULL)
                    .set_style(THEME.normal);
            }
            if ratio < 1.0 {
                fn get_unicode_block<'a>(frac: f64) -> &'a str {
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
