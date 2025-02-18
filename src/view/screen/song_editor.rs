use std::iter;

use itertools::izip;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Style,
    text::Line,
    Frame,
};

use crate::{
    model,
    view::{theme::THEME, widget::pattern_line::PatternLineView},
};

const CHANNEL_HEADER_HEIGHT: u16 = 1;
const CHANNEL_HORIZONTAL_PADDING: u16 = 1;
const CHANNEL_TOTAL_HORIZONTAL_PADDING: u16 = CHANNEL_HORIZONTAL_PADDING * 2; // Left + Right
const CHANNEL_CONTENT_WIDTH: u16 = PatternLineView::LINE_WIDTH;
const CHANNEL_TOTAL_WIDTH: u16 = CHANNEL_CONTENT_WIDTH + CHANNEL_TOTAL_HORIZONTAL_PADDING;

fn compute_three_states_scrolling(
    view_size: usize,
    total_size: usize,
    cursor_position: usize,
) -> usize {
    let half_height = (view_size as f32 / 2.0).round() as usize;
    let scroll_lower_bound = half_height;
    let scroll_upper_bound = total_size - half_height;

    if cursor_position < scroll_lower_bound {
        0
    } else if cursor_position >= scroll_upper_bound {
        total_size - view_size
    } else {
        scroll_lower_bound.abs_diff(cursor_position) + 1
    }
}

pub fn render(frame: &mut Frame, area: Rect, state: &model::State) {
    // TODO: compute line numbers area from state instead of 3
    let [line_numbers_area, pattern_area] =
        Layout::horizontal([Constraint::Length(3), Constraint::Fill(1)])
            .spacing(1)
            .areas(area);

    let [_, line_numbers_area] = Layout::vertical([
        Constraint::Length(CHANNEL_HEADER_HEIGHT),
        Constraint::Fill(1),
    ])
    .areas(line_numbers_area);

    let [_, pattern_scroll_area] = Layout::vertical([
        Constraint::Length(CHANNEL_HEADER_HEIGHT),
        Constraint::Fill(1),
    ])
    .areas(pattern_area);

    let channel_len = state.patterns.channel_len as usize;

    let vertical_offset = compute_three_states_scrolling(
        pattern_scroll_area.height as usize,
        channel_len,
        state.patterns.current_row as usize,
    );

    let currently_playing_row = state
        .currently_played_line()
        .filter(|_| state.is_song_playing());

    (vertical_offset..channel_len)
        .map(|line_number| {
            Line::raw(format!("{}", line_number)).right_aligned().style(
                if currently_playing_row
                    .is_some_and(|current_playing_row| current_playing_row == line_number)
                {
                    THEME.secondary_cursor
                } else {
                    Style::reset()
                },
            )
        })
        .zip(line_numbers_area.rows())
        .for_each(|(line_widget, line_number_area)| {
            frame.render_widget(line_widget, line_number_area)
        });

    let displayed_channel_count =
        (pattern_scroll_area.width + CHANNEL_TOTAL_HORIZONTAL_PADDING) / CHANNEL_TOTAL_WIDTH;
    let displayed_channel_count = displayed_channel_count as usize;

    let channel_offset = compute_three_states_scrolling(
        displayed_channel_count,
        state.patterns.channel_count as usize,
        state.patterns.current_channel as usize,
    );

    let channels_areas = Layout::horizontal(
        iter::repeat(CHANNEL_CONTENT_WIDTH)
            .take(displayed_channel_count)
            .map(Constraint::Length),
    )
    .spacing(CHANNEL_TOTAL_HORIZONTAL_PADDING)
    .split(pattern_area);

    let channels = state
        .patterns
        .current_pattern_channels()
        .skip(channel_offset)
        .take(displayed_channel_count);

    for (channel_lines, channel_area, channel_index) in izip!(
        channels,
        channels_areas.iter(),
        channel_offset..state.patterns.channel_count as usize,
    ) {
        debug_assert_eq!(state.patterns.channel_len as usize, channel_lines.len());
        let [header_area, lines_area] = Layout::vertical([
            Constraint::Length(CHANNEL_HEADER_HEIGHT),
            Constraint::Fill(1),
        ])
        .areas(*channel_area);

        frame.render_widget(
            Line::raw(format!("Track {}", channel_index + 1)).centered(),
            header_area,
        );

        let displayed_line_count = channel_lines
            .len()
            .saturating_sub(vertical_offset)
            .min(area.height as usize);

        let line_start = vertical_offset;
        let line_end = vertical_offset + displayed_line_count;
        let lines = &channel_lines[line_start..line_end];

        let [lines_area] = Layout::horizontal([Constraint::Length(PatternLineView::LINE_WIDTH)])
            .flex(Flex::Center)
            .areas(lines_area);

        for (line_index, line, area) in izip!(
            vertical_offset..state.patterns.channel_len as usize,
            lines,
            lines_area.rows(),
        ) {
            frame.render_widget(
                PatternLineView {
                    line,
                    current_field: (state.patterns.current_channel == channel_index as i32)
                        .then_some(state.patterns.current_field),
                    is_line_selected: state.patterns.current_row as usize == line_index,
                    is_line_played: currently_playing_row
                        .is_some_and(|current_playing_row| line_index == current_playing_row),
                },
                area,
            );
        }
    }
}
