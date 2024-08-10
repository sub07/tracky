use itertools::multizip;
use joy_macro::New;
use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer, Rect},
    text::Line,
    widgets::Widget,
};

use crate::model::pattern::PatternLine;

use super::line::PatternLineView;

#[derive(New)]
pub struct ChannelView<'a> {
    pub lines: &'a [PatternLine],
    pub row_offset: usize,
    pub channel_index: usize,
    pub current_row: i32,
    pub current_field: Option<i32>,
    pub channel_len: i32,
}

impl ChannelView<'_> {
    const TITLE_BOX_HEIGHT: u16 = 1;
    pub const PADDING: u16 = 2;
    pub const WIDTH: u16 = PatternLineView::LINE_WIDTH;

    fn title_widget(&self) -> Line {
        Line::raw(format!("{}", self.channel_index + 1)).centered()
    }

    pub fn layout(area: Rect) -> [Rect; 2] {
        Layout::vertical([
            Constraint::Length(Self::TITLE_BOX_HEIGHT),
            Constraint::Percentage(100),
        ])
        .areas(area)
    }
}

impl Widget for ChannelView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        debug_assert_eq!(self.channel_len as usize, self.lines.len());

        let [title_area, lines_area] = Self::layout(area);

        self.title_widget().render(title_area, buf);

        let displayed_line_count = self
            .lines
            .len()
            .saturating_sub(self.row_offset)
            .min(area.height as usize);

        let line_start = self.row_offset;
        let line_end = self.row_offset + displayed_line_count;
        let lines = &self.lines[line_start..line_end];

        let [lines_area] = Layout::horizontal([Constraint::Length(PatternLineView::LINE_WIDTH)])
            .flex(Flex::Center)
            .areas(lines_area);

        // TODO Replace multizip with izip!() when fixed : https://github.com/rust-lang/rust-analyzer/issues/11681
        for (line_index, line, area) in multizip((
            (self.row_offset as i32..self.channel_len),
            lines,
            lines_area.rows(),
        )) {
            PatternLineView {
                line,
                current_field: self.current_field,
                is_line_selected: self.current_row == line_index,
            }
            .render(area, buf)
        }
    }
}
