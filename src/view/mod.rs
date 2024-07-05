use pattern::PatternView;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Widget,
    Frame,
};

use crate::{log::render_log_panel, tracky::Tracky};

pub mod channel;
pub mod line;
pub mod pattern;

pub fn render_root(app: &mut Tracky, frame: &mut Frame) {
    let area = frame.size();

    let pattern_view = PatternView::new(
        app.patterns.current_pattern_channels(),
        app.patterns.current_row,
        app.patterns.current_channel,
        app.patterns.current_field,
        app.patterns.channel_len,
        app.patterns.channel_count,
    );

    pattern_view.render(area, frame.buffer_mut());

    if app.display_log_console {
        let [_, console_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Percentage(30)]).areas(area);

        let [_, console_area] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Min(50)]).areas(console_area);

        render_log_panel(frame, console_area);
    }
}
