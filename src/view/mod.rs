use pattern::PatternView;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
    Frame,
};

use crate::{log::render_log_panel, tracky::Tracky};

pub mod channel;
pub mod line;
pub mod pattern;
pub mod popup;

fn centered_rect(area: Rect, width: Constraint, height: Constraint) -> Rect {
    let [_, center, _] =
        Layout::horizontal([Constraint::Fill(1), width, Constraint::Fill(1)]).areas(area);

    let [_, center, _] =
        Layout::vertical([Constraint::Fill(1), height, Constraint::Fill(1)]).areas(center);

    center
}

fn center_row(area: Rect) -> Rect {
    let [_, center, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(area);

    center
}

fn clamp_layout_width(area: Rect, value: Constraint, min: Constraint, max: Constraint) -> Rect {
    let [_, wanted_area, _] =
        Layout::horizontal([Constraint::Fill(1), value, Constraint::Fill(1)]).areas(area);
    let [_, min_area, _] =
        Layout::horizontal([Constraint::Fill(1), min, Constraint::Fill(1)]).areas(area);
    let [_, max_area, _] =
        Layout::horizontal([Constraint::Fill(1), max, Constraint::Fill(1)]).areas(area);

    if wanted_area.width > max_area.width {
        max_area
    } else if wanted_area.width < min_area.width {
        min_area
    } else {
        wanted_area
    }
}

pub fn render_root(app: &mut Tracky, frame: &mut Frame) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let pattern_view = PatternView::new(
        app.patterns.current_pattern_channels(),
        app.patterns.current_row,
        app.patterns.current_channel,
        app.patterns.current_field,
        app.patterns.channel_len,
        app.patterns.channel_count,
    );

    pattern_view.render(area, buf);

    if let Some(popup) = &mut app.popup_state {
        match popup {
            popup::Popup::AudioDeviceSelection(popup) => popup.render(area, buf),
        }
    }

    if app.display_log_console {
        let [_, console_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Percentage(30)]).areas(area);

        let [_, console_area] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Min(50)]).areas(console_area);

        render_log_panel(frame, console_area);
    }
}
