use std::iter;

use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Flex, Layout},
    text::Line,
    widgets::Widget,
};

pub struct Header<'l> {
    pub lines: Vec<Line<'l>>,
}

impl<'l> Header<'l> {
    pub fn new<I>(lines: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Line<'l>>,
    {
        Self {
            lines: lines.into_iter().map(Into::into).collect(),
        }
    }
}

impl Widget for Header<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let last_index = self.lines.len() - 1;

        let lines_rect = Layout::horizontal(iter::repeat_n(Constraint::Fill(1), self.lines.len()))
            .flex(Flex::SpaceBetween)
            .split(area)
            .iter()
            .enumerate()
            .zip(self.lines.iter())
            .map(|((index, area), line)| {
                if index == 0 {
                    let [area, _] = Layout::horizontal([
                        Constraint::Length(line.width() as u16),
                        Constraint::Fill(1),
                    ])
                    .areas(*area);
                    area
                } else if index == last_index {
                    let [_, area] = Layout::horizontal([
                        Constraint::Fill(1),
                        Constraint::Length(line.width() as u16),
                    ])
                    .areas(*area);
                    area
                } else {
                    let [_, area, _] = Layout::horizontal([
                        Constraint::Fill(1),
                        Constraint::Length(line.width() as u16),
                        Constraint::Fill(1),
                    ])
                    .areas(*area);
                    area
                }
            })
            .collect_vec();

        for (line, area) in self.lines.into_iter().zip(lines_rect) {
            line.render(area, buf);
        }
    }
}
