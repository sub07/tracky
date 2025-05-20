use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, ToSpan},
    widgets::Widget,
    Frame,
};
use theme::THEME;
use widget::header::Header;

use crate::tracky::Tracky;

pub mod popup;
pub mod screen;
pub mod theme;
pub mod widget;

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

#[allow(dead_code)]
pub fn debug_area(frame: &mut Frame, area: Rect, color: Color) {
    let buffer = frame.buffer_mut();

    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            buffer.cell_mut((x, y)).unwrap().set_bg(color);
        }
    }
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
    let [header_area, area] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());

    let audio_state_text = Line::from_iter([
        "• ".fg(if app.audio_state.is_some() {
            THEME.success
        } else {
            THEME.danger
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

    let playback_state_text = Line::from(if app.state.is_song_playing() {
        "Playing"
    } else {
        "Not playing"
    });

    frame.render_widget(
        Header::new([
            audio_state_text,
            playback_state_text,
            Line::from_iter([
                "Update per second: ".to_span(),
                app.stats.update_per_second().to_span(),
            ]),
        ]),
        header_area,
    );

    match &mut app.current_screen {
        screen::Screen::DeviceSelection(device_selection_screen_state) => {
            device_selection_screen_state.render(area, frame.buffer_mut())
        }
        screen::Screen::SongEditor => {
            screen::song_editor::render(frame, area, &app.state);
        }
    }

    for popup in app.current_popup.iter_mut() {
        match popup {
            popup::Popup::Input(popup) => popup.render(area, frame.buffer_mut()),
        }
    }

    if app.loader_count > 0 {
        // TODO remove buffer usage, use Frame for consistency
        popup::loading::render(area, frame.buffer_mut());
    }
}
