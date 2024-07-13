use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    sync::Mutex,
};

use itertools::Itertools;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Text},
    widgets::{block::Title, Block, BorderType, Clear, Paragraph},
    Frame,
};

#[derive(Clone)]
enum LogLevel {
    Error,
    Info,
    Debug,
}

#[derive(Clone)]
struct LogEntry {
    tag: &'static str,
    content: String,
    level: LogLevel,
    line_count: usize,
}

static LOG_ENTRIES: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());

fn alter_entries<F>(f: F)
where
    F: FnOnce(&mut Vec<LogEntry>),
{
    f(LOG_ENTRIES.lock().unwrap().deref_mut());
}

fn read_entries<T, F>(f: F) -> T
where
    F: FnOnce(&[LogEntry]) -> T,
{
    f(LOG_ENTRIES.lock().unwrap().deref())
}

fn add_entry(tag: &'static str, content: String, level: LogLevel) {
    let line_count = content.lines().count();
    alter_entries(|logs| {
        logs.push(LogEntry {
            tag,
            content,
            level,
            line_count,
        })
    })
}

pub trait DisplayLogExt {
    fn info(&self, tag: &'static str);
}

pub fn info<D: Display>(tag: &'static str, content: &D) {
    add_entry(tag, content.to_string(), LogLevel::Info);
}

impl<T: Display> DisplayLogExt for T {
    fn info(&self, tag: &'static str) {
        info(tag, self);
    }
}

pub trait DebugLogExt {
    fn debug(self, tag: &'static str) -> Self;
    fn error(&self, tag: &'static str);
}

pub fn debug<D: Debug>(tag: &'static str, content: &D) {
    add_entry(tag, format!("{content:#?}"), LogLevel::Debug);
}

pub fn error<D: Debug>(tag: &'static str, content: &D) {
    add_entry(tag, format!("{content:#?}"), LogLevel::Error);
}

impl<T: Debug> DebugLogExt for T {
    fn debug(self, tag: &'static str) -> Self {
        debug(tag, &self);
        self
    }

    fn error(&self, tag: &'static str) {
        error(tag, self)
    }
}

pub fn clear_entries() {
    alter_entries(|logs| logs.clear());
}

// TODO check if intersperse is added to stdlib to replace itertools usage and remove unstable_name_collisions lint bypass
#[allow(unstable_name_collisions)]
pub fn render_log_panel(frame: &mut Frame, area: Rect) {
    // render_empty_log_panel(frame, area);
    // return;

    let block = Block::bordered().border_type(BorderType::Rounded).title(
        Title::from(" <F9> Save on Disk - <F10> Clear - <F12> Toggle ")
            .alignment(Alignment::Center),
    );

    let available_line_count = block.inner(area).height as usize;
    if available_line_count == 0 {
        return;
    }

    let (mut entries, line_count) = read_entries(|logs| {
        let mut line_count = 0;
        (
            logs.iter()
                .cloned()
                .rev()
                .take_while(|entry| {
                    let take = line_count <= available_line_count;
                    line_count += entry.line_count;
                    take
                })
                .collect_vec(),
            line_count,
        )
    });

    if entries.is_empty() {
        render_empty_log_panel(frame, area);
        return;
    }

    if line_count > available_line_count {
        let oldest_entry = entries.last_mut().unwrap();
        oldest_entry.content = oldest_entry
            .content
            .lines()
            .skip(line_count - available_line_count)
            .intersperse("\n")
            .collect::<String>()
    }

    let text = entries
        .iter()
        .rev()
        .flat_map(|entry| {
            entry.content.lines().enumerate().map(|(line_index, line)| {
                if line_index == 0 {
                    let tag_color = match entry.level {
                        LogLevel::Error => Color::Red,
                        LogLevel::Info => Color::LightBlue,
                        LogLevel::Debug => Color::DarkGray,
                    };
                    Line::from_iter([entry.tag.fg(tag_color).italic(), " ".into(), line.into()])
                } else {
                    Line::raw(line)
                }
            })
        })
        .collect::<Text>();

    frame.render_widget(Clear, area);
    frame.render_widget(Paragraph::new(text).block(block), area);
}

fn render_empty_log_panel(frame: &mut Frame, area: Rect) {
    let block = Block::bordered().border_type(BorderType::Rounded).title(
        Title::from(" <F9> Save on Disk - <F10> Clear - <F12> Toggle ")
            .alignment(Alignment::Center),
    );

    let inner_area = block.inner(area);

    let [_, text_area, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(inner_area);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget("No logs yet".dark_gray().into_centered_line(), text_area);
}