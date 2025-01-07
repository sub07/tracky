use std::iter;

use itertools::izip;
use joy_macro::New;
use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::Widget,
};

use crate::model::pattern::PatternLine;

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

        // TODO better vertical scroll
        let scroll_ratio = self.current_row as f32 / (self.channel_len - 1) as f32;
        let below_scroll_height = self.channel_len - channel_pattern_lines_area.height as i32;
        let row_offset = (below_scroll_height as f32 * scroll_ratio) as usize;

        let [_, line_numbers_area] = Layout::vertical([
            Constraint::Length(channel_title_area.height),
            Constraint::Fill(1),
        ])
        .areas(line_numbers_area);

        (row_offset..self.channel_len as usize)
            .map(|line_number| Line::raw(format!("{}", line_number)).right_aligned())
            .zip(line_numbers_area.rows())
            .for_each(|(line_widget, line_number_area)| line_widget.render(line_number_area, buf));

        const CHANNEL_PADDING: i32 = ChannelView::PADDING as i32;
        const CHANNEL_TOTAL_WIDTH: i32 = ChannelView::WIDTH as i32 + CHANNEL_PADDING;

        let displayed_channel_count =
            (pattern_area.width as i32 + CHANNEL_PADDING) / CHANNEL_TOTAL_WIDTH;

        // TODO better horizontal scroll
        let scroll_ratio = self.current_channel as f32 / (self.channel_count - 1) as f32;
        let after_scroll_channel_count = self.channel_count - displayed_channel_count;
        let channel_offset = (after_scroll_channel_count as f32 * scroll_ratio) as usize;

        let displayed_channel_count = displayed_channel_count as usize;

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
                row_offset,
                channel_index,
                self.current_row,
                (self.current_channel == channel_index as i32).then_some(self.current_field),
                self.channel_len,
            );

            channel_view.render(*channel_area, buf);
        }
    }
}
