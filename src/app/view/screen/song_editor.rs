use itertools::{Itertools, izip};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::Paragraph,
};

use crate::{
    app::view::{debug_area, theme::THEME, widget::pattern_line::PatternLineView},
    assert_log, model,
};

const CHANNEL_HEADER_HEIGHT: u16 = 1;
const CHANNEL_HORIZONTAL_PADDING: u16 = 1;
const CHANNEL_TOTAL_HORIZONTAL_PADDING: u16 = CHANNEL_HORIZONTAL_PADDING * 2; // Left + Right
const CHANNEL_CONTENT_WIDTH: u16 = PatternLineView::LINE_WIDTH;
const CHANNEL_TOTAL_WIDTH: u16 = CHANNEL_CONTENT_WIDTH + CHANNEL_TOTAL_HORIZONTAL_PADDING;

struct RootLayout {
    line_numbers_area: Rect,
    channel_areas: Vec<ChannelLayout>,
    channels_area: Rect,
}

fn channel_layout(is_debug_enabled: bool) -> Layout {
    if is_debug_enabled {
        Layout::vertical([
            Constraint::Length(CHANNEL_HEADER_HEIGHT),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
    } else {
        Layout::vertical([
            Constraint::Length(CHANNEL_HEADER_HEIGHT),
            Constraint::Fill(1),
        ])
    }
}

impl RootLayout {
    pub fn new(area: Rect, pattern_line_count: usize, is_debug_enabled: bool) -> Self {
        let [line_numbers_area, channels_area] = Layout::horizontal([
            Constraint::Length(pattern_line_count.to_string().len() as u16),
            Constraint::Fill(1),
        ])
        .spacing(1)
        .areas(area);

        let line_numbers_area = channel_layout(is_debug_enabled)
            .split(line_numbers_area)
            .last()
            .copied()
            .unwrap();

        let displayed_channel_count =
            (channels_area.width + CHANNEL_TOTAL_HORIZONTAL_PADDING) / CHANNEL_TOTAL_WIDTH;

        let channel_areas = Layout::horizontal(
            std::iter::repeat_n(CHANNEL_CONTENT_WIDTH, displayed_channel_count as usize)
                .map(Constraint::Length),
        )
        .spacing(CHANNEL_TOTAL_HORIZONTAL_PADDING)
        .split(channels_area)
        .iter()
        .map(|channel_area| ChannelLayout::new(*channel_area, is_debug_enabled))
        .collect_vec();

        Self {
            line_numbers_area,
            channel_areas,
            channels_area,
        }
    }

    fn pattern_scroll_height(&self) -> u16 {
        self.channel_areas
            .first()
            .expect("Cannot have no channel")
            .lines_area
            .height
    }

    fn debug(&self, frame: &mut Frame) {
        debug_area(frame, self.channels_area, Color::Blue);
        debug_area(frame, self.line_numbers_area, Color::Red);

        self.channel_areas.iter().for_each(|channel_layout| {
            debug_area(frame, channel_layout.header_area, Color::Green);
            if let Some(debug) = channel_layout.debug_area {
                debug_area(frame, debug, Color::Yellow);
            }
            debug_area(frame, channel_layout.lines_area, Color::Magenta);
        });
    }
}

struct ChannelLayout {
    header_area: Rect,
    debug_area: Option<Rect>,
    lines_area: Rect,
}

impl ChannelLayout {
    fn new(area: Rect, is_debug_enabled: bool) -> Self {
        let areas = channel_layout(is_debug_enabled).split(area);
        if areas.len() == 3 && is_debug_enabled {
            Self {
                header_area: areas[0],
                debug_area: Some(areas[1]),
                lines_area: areas[2],
            }
        } else if areas.len() == 2 && !is_debug_enabled {
            Self {
                header_area: areas[0],
                debug_area: None,
                lines_area: areas[1],
            }
        } else {
            panic!("Unexpected area layout configuration");
        }
    }
}

fn compute_three_states_scrolling(
    view_size: usize,
    total_size: usize,
    cursor_position: usize,
) -> usize {
    let half_height = (view_size as f32 / 2.0).round() as usize;
    let scroll_lower_bound = half_height;
    let scroll_upper_bound = total_size.saturating_sub(half_height);

    if cursor_position < scroll_lower_bound {
        0
    } else if cursor_position >= scroll_upper_bound {
        total_size.saturating_sub(view_size)
    } else {
        scroll_lower_bound.abs_diff(cursor_position) + 1
    }
}

pub fn render(frame: &mut Frame, area: Rect, state: &model::State) {
    let layout = RootLayout::new(
        area,
        state.patterns.channel_len as usize,
        cfg!(debug_assertions),
    );

    let channel_len = state.patterns.channel_len as usize;

    let vertical_offset = compute_three_states_scrolling(
        layout.pattern_scroll_height() as usize,
        channel_len,
        state.patterns.current_row as usize,
    );

    let currently_playing_row = state
        .currently_played_line()
        .filter(|_| state.is_song_playing());

    (vertical_offset..channel_len)
        .map(|line_number| render_line_number(currently_playing_row, line_number))
        .zip(layout.line_numbers_area.rows())
        .for_each(|(line_widget, line_number_area)| {
            frame.render_widget(line_widget, line_number_area);
        });

    let channel_offset = compute_three_states_scrolling(
        layout.channel_areas.len(),
        state.patterns.channel_count as usize,
        state.patterns.current_channel as usize,
    );

    let channels = state
        .patterns
        .current_pattern_channels()
        .skip(channel_offset)
        .take(layout.channel_areas.len());

    for (channel_lines, channel_layout, channel_index) in izip!(
        channels,
        layout.channel_areas.iter(),
        channel_offset..state.patterns.channel_count as usize,
    ) {
        render_channel(
            frame,
            state,
            vertical_offset,
            currently_playing_row,
            channel_lines,
            channel_layout,
            channel_index,
        );
    }
}

fn render_channel(
    frame: &mut Frame<'_>,
    state: &model::State,
    vertical_offset: usize,
    currently_playing_row: Option<usize>,
    channel_lines: &[model::pattern::PatternLine],
    ChannelLayout {
        header_area,
        debug_area,
        lines_area,
    }: &ChannelLayout,
    channel_index: usize,
) {
    assert_log!(state.patterns.channel_len as usize == channel_lines.len());

    frame.render_widget(
        Line::raw(format!("Track {}", channel_index + 1)).centered(),
        *header_area,
    );

    let channel = &state.channels[channel_index];

    // If debug build then debug info
    if let Some(debug_area) = debug_area {
        frame.render_widget(
            Paragraph::new(Text::from(vec![
                Line::from(format!(
                    "Instr:{}",
                    match channel.instrument {
                        Some(ref instrument) => instrument.index.to_string(),
                        None => "-".to_string(),
                    }
                )),
                Line::from(match channel.instrument {
                    Some(ref instrument) => instrument.phase.to_string(),
                    None => "-".to_string(),
                }),
                Line::from(format!(
                    "Note :{}",
                    match channel.note {
                        Some((note, octave)) => format!("{note}{}", octave.value()),
                        None => "-".to_string(),
                    }
                )),
            ])),
            *debug_area,
        );
    }

    let displayed_line_count = channel_lines
        .len()
        .saturating_sub(vertical_offset)
        .min(lines_area.height as usize);

    let line_start = vertical_offset;
    let line_end = vertical_offset + displayed_line_count;
    let lines = &channel_lines[line_start..line_end];

    let [lines_area] = Layout::horizontal([Constraint::Length(PatternLineView::LINE_WIDTH)])
        .flex(Flex::Center)
        .areas(*lines_area);

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

fn render_line_number(currently_playing_row: Option<usize>, line_number: usize) -> Line<'static> {
    Line::raw(format!("{line_number}")).right_aligned().style(
        if currently_playing_row
            .is_some_and(|current_playing_row| current_playing_row == line_number)
        {
            THEME.secondary_cursor
        } else {
            Style::reset()
        },
    )
}
