use header::Header;
use pattern::PatternView;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::Widget,
    Frame,
};

use crate::tracky::Tracky;

pub mod channel;
pub mod header;
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

fn responsive_centered_rect(
    area: Rect,
    prefered_width: Constraint,
    min_width: Constraint,
    max_width: Constraint,
    height: Constraint,
) -> Rect {
    let area = clamp_layout_width(area, prefered_width, min_width, max_width);
    centered_rows(area, height)
}

fn centered_line(area: Rect) -> Rect {
    centered_rows(area, Constraint::Length(1))
}

fn centered_rows(area: Rect, height: Constraint) -> Rect {
    let [_, height, _] =
        Layout::vertical([Constraint::Fill(1), height, Constraint::Fill(1)]).areas(area);
    height
}

fn margin(area: Rect, margin: u16) -> Rect {
    let [area] = Layout::default()
        .constraints([Constraint::Min(0)])
        .margin(margin)
        .areas(area);
    area
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
        app.state.patterns.current_pattern_channels(),
        app.state.patterns.current_row,
        app.state.patterns.current_channel,
        app.state.patterns.current_field,
        app.state.patterns.channel_len,
        app.state.patterns.channel_count,
    );

    let [header_area, pattern_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);

    let audio_state_text = Line::from_iter([
        "• ".fg(if app.audio_state.is_some() {
            Color::LightGreen
        } else {
            Color::LightRed
        }),
        if app.audio_state.is_some() {
            match &app.selected_output_device {
                Some(device) => device.name.clone(),
                None => "Default device".to_string(),
            }
            .into()
        } else {
            "No audio".into()
        },
    ]);

    let playback_state_text = Line::from(if app.state.is_playing() {
        "Playing"
    } else {
        "Not playing"
    });

    // Header::new([audio_state_text, playback_state_text]).render(header_area, buf);

    // pattern_view.render(pattern_area, buf);

    for popup in app.popup_state.iter_mut() {
        match popup {
            popup::Popup::AudioDeviceSelection(popup) => popup.render(area, buf),
            popup::Popup::Input(popup) => popup.render(area, buf),
        }
    }

    if app.loader_count > 0 {
        popup::loading::Popup.render(area, buf);
    }
}
