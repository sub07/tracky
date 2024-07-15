use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Mutex,
};

use eyre::eyre;
use itertools::Itertools;
use joy_macro::EnumStr;
use log::debug;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Text},
    widgets::{block::Title, Block, BorderType, Clear, Paragraph},
    Frame,
};

#[derive(Clone, Copy, Debug, EnumStr)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

#[derive(Clone)]
struct LogEntry {
    content: String,
    level: LogLevel,
    line_count: usize,
}

#[derive(Default)]
struct TerminalLogger {
    entries: Vec<LogEntry>,
}

static TERMINAL_LOGGER: Mutex<TerminalLogger> = Mutex::new(TerminalLogger {
    entries: Vec::new(),
});

fn alter_entries<F>(f: F)
where
    F: FnOnce(&mut TerminalLogger),
{
    f(TERMINAL_LOGGER.lock().unwrap().deref_mut());
}

fn read_entries<T, F>(f: F) -> T
where
    F: FnOnce(&TerminalLogger) -> T,
{
    f(TERMINAL_LOGGER.lock().unwrap().deref())
}

fn add_entry(content: String, level: LogLevel) {
    let line_count = content.lines().count();
    alter_entries(|logs| {
        logs.entries.push(LogEntry {
            content,
            level,
            line_count,
        })
    })
}

pub trait DebugLogExt {
    #[allow(dead_code)] // because it's dev utils
    fn debug(self, tag: &'static str) -> Self;
}

impl<T: Debug> DebugLogExt for T {
    fn debug(self, tag: &'static str) -> Self {
        debug!("{tag}: {:?}", &self);
        self
    }
}

pub fn clear_entries() {
    alter_entries(|logs| logs.entries.clear());
}

// TODO check if intersperse is added to stdlib to replace itertools usage and remove unstable_name_collisions lint bypass
#[allow(unstable_name_collisions)]
pub fn render_log_panel(frame: &mut Frame, area: Rect) {
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
            logs.entries
                .iter()
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
                        LogLevel::Warn => Color::Yellow,
                    };
                    Line::from_iter([
                        entry.level.as_str().fg(tag_color).italic(),
                        " ".into(),
                        line.into(),
                    ])
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

// Sorry future me
struct DummyLogger;
static DUMMY_LOGGER: DummyLogger = DummyLogger;

impl log::Log for DummyLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = match record.level() {
            log::Level::Error => LogLevel::Error,
            log::Level::Warn => LogLevel::Warn,
            log::Level::Info => LogLevel::Info,
            log::Level::Debug | log::Level::Trace => LogLevel::Debug,
        };

        add_entry(format!("{}", record.args()), level);
    }

    fn flush(&self) {}
}

pub fn setup() -> eyre::Result<()> {
    log::set_logger(&DUMMY_LOGGER)
        .map_err(|_| eyre!("Error while setting up logger: maybe setup called twice"))?;
    log::set_max_level(log::LevelFilter::Trace);
    Ok(())
}
