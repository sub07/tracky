use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    text::ToLine,
    widgets::{Block, Clear, Widget},
};

use crate::view::{centered_line, responsive_centered_rect};

pub struct Popup;

impl Popup {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let area = responsive_centered_rect(
            area,
            Constraint::Percentage(30),
            Constraint::Length(30),
            Constraint::Length(50),
            Constraint::Percentage(30),
        );

        let block = Block::bordered();

        let area = {
            let inner = block.inner(area);
            Clear.render(area, buf);
            block.render(area, buf);
            inner
        };

        let area = centered_line(area);
        "Loading...".to_line().centered().render(area, buf);
    }
}
