use ratatui::{
    layout::{Constraint, Rect},
    text::ToLine,
    widgets::Block,
    Frame,
};

use crate::view::{centered_line, render_block_and_get_inner, responsive_centered_rect};

pub fn render(frame: &mut Frame, area: Rect) {
    let area = responsive_centered_rect(
        area,
        Constraint::Percentage(30),
        Constraint::Length(30),
        Constraint::Length(50),
        Constraint::Percentage(30),
    );

    let area = render_block_and_get_inner(Block::bordered(), frame, area);

    let area = centered_line(area);
    frame.render_widget("Loading...".to_line().centered(), area);
}
