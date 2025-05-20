use std::iter;

use itertools::izip;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Style,
    text::{Line, Text},
    widgets::Paragraph,
    Frame,
};

use crate::{
    assert_log, model,
    view::{theme::THEME, widget::pattern_line::PatternLineView},
};

const CHANNEL_HEADER_HEIGHT: u16 = 1;
const CHANNEL_HORIZONTAL_PADDING: u16 = 1;
const CHANNEL_TOTAL_HORIZONTAL_PADDING: u16 = CHANNEL_HORIZONTAL_PADDING * 2; // Left + Right
const CHANNEL_CONTENT_WIDTH: u16 = PatternLineView::LINE_WIDTH;
const CHANNEL_TOTAL_WIDTH: u16 = CHANNEL_CONTENT_WIDTH + CHANNEL_TOTAL_HORIZONTAL_PADDING;

fn channel_layout() -> Layout {
    Layout::vertical([
        Constraint::Length(CHANNEL_HEADER_HEIGHT),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .spacing(1)
}

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
    let [line_numbers_area, pattern_area] = Layout::horizontal([
        Constraint::Length(state.patterns.channel_len.to_string().len() as u16),
        Constraint::Fill(1),
    ])
    .spacing(1)
    .areas(area);
    let channel_layout = channel_layout();
    let [_, _, line_numbers_area] = channel_layout.areas(line_numbers_area);
    let [_, _, pattern_scroll_area] = channel_layout.areas(pattern_area);

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
        std::iter::repeat_n(CHANNEL_CONTENT_WIDTH, displayed_channel_count).map(Constraint::Length),
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
        assert_log!(state.patterns.channel_len as usize == channel_lines.len());

        let [header_area, debug_area, lines_area] = channel_layout.areas(*channel_area);

        frame.render_widget(
            Line::raw(format!("Track {}", channel_index + 1)).centered(),
            header_area,
        );

        let channel = &state.channels[channel_index];

        // If debug build then debug info
        if cfg!(debug_assertions) {
            frame.render_widget(
                Paragraph::new(Text::from(vec![
                    Line::from(format!(
                        "Instr:{}",
                        match channel.current_instrument {
                            Some(ref instrument) => instrument.index.to_string(),
                            None => "-".to_string(),
                        }
                    )),
                    Line::from(
                        (match channel.current_instrument {
                            Some(ref instrument) => instrument.phase.to_string(),
                            None => "-".to_string(),
                        })
                        .to_string(),
                    ),
                    Line::from(format!(
                        "Note :{}",
                        match channel.current_note {
                            Some((note, octave)) => format!("{note}{}", octave.value()),
                            None => "-".to_string(),
                        }
                    )),
                ])),
                debug_area,
            );
        }

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
