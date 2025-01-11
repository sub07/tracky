use std::iter;

use itertools::izip;
use joy_macro::New;
use log::debug;
use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::Widget,
};

use crate::{log::DebugLogExt, model::pattern::PatternLine};

use super::channel::ChannelView;

#[derive(New)]
pub struct PatternView<'a, I>
where
    I: Iterator<Item = &'a [PatternLine]>,
{
    channels: I,
    current_row: i32,
    current_channel: i32,
    current_field: i32,
    channel_len: i32,
    channel_count: i32,
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
    } else if cursor_position > scroll_upper_bound {
        total_size - view_size
    } else {
        scroll_lower_bound.abs_diff(cursor_position) + 1
    }
}

impl<'a, I> Widget for PatternView<'a, I>
where
    I: Iterator<Item = &'a [PatternLine]>,
{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [line_numbers_area, pattern_area] =
            Layout::horizontal([Constraint::Length(3), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        let [channel_title_area, channel_pattern_lines_area] = ChannelView::layout(area);

        let [_, line_numbers_area] = Layout::vertical([
            Constraint::Length(channel_title_area.height),
            Constraint::Fill(1),
        ])
        .areas(line_numbers_area);

        let vertical_offset = compute_three_states_scrolling(
            channel_pattern_lines_area.height as usize,
            self.channel_len as usize,
            self.current_row as usize,
        );

        (vertical_offset..self.channel_len as usize)
            .map(|line_number| Line::raw(format!("{}", line_number)).right_aligned())
            .zip(line_numbers_area.rows())
            .for_each(|(line_widget, line_number_area)| line_widget.render(line_number_area, buf));

        const CHANNEL_PADDING: i32 = ChannelView::PADDING as i32;
        const CHANNEL_TOTAL_WIDTH: i32 = ChannelView::WIDTH as i32 + CHANNEL_PADDING;

        let displayed_channel_count =
            (pattern_area.width as i32 + CHANNEL_PADDING) / CHANNEL_TOTAL_WIDTH;
        let displayed_channel_count = displayed_channel_count as usize;

        let channel_offset = compute_three_states_scrolling(
            displayed_channel_count,
            self.channel_count as usize,
            self.current_channel as usize,
        );

        let channel_areas = Layout::horizontal(
            iter::repeat(ChannelView::WIDTH)
                .take(displayed_channel_count)
                .map(Constraint::Length),
        )
        .spacing(ChannelView::PADDING)
        .split(pattern_area);

        let channels = self
            .channels
            .skip(channel_offset)
            .take(displayed_channel_count);

        for (channel, channel_area, channel_index) in izip!(
            channels,
            channel_areas.iter(),
            channel_offset..self.channel_count as usize,
        ) {
            let channel_view = ChannelView::new(
                channel,
                vertical_offset,
                channel_index,
                self.current_row,
                (self.current_channel == channel_index as i32).then_some(self.current_field),
                self.channel_len,
            );

            channel_view.render(*channel_area, buf);
        }
    }
}
