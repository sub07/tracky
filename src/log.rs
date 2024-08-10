use std::{
    fmt::Debug,
    fs,
    ops::{Deref, DerefMut},
    path::Path,
    sync::Mutex,
};

use anyhow::{anyhow, Context};
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

pub fn write_logs_to_file<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let logs = read_entries(|logger| {
        logger
            .entries
            .iter()
            .map(|entry| format!("{:?} - {}", entry.level, entry.content))
            .join("\n")
    });

    fs::write(&path, logs).with_context(|| format!("{:?}", path.as_ref()))?;
    Ok(())
}

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
    alter_entries(|logger| {
        logger.entries.push(LogEntry {
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
    alter_entries(|logger| logger.entries.clear());
}

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

    let (mut entries, line_count) = read_entries(|logger| {
        let mut line_count = 0;
        (
            logger
                .entries
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

pub fn setup() -> anyhow::Result<()> {
    log::set_logger(&DUMMY_LOGGER)
        .map_err(|_| anyhow!("Error while setting up logger: maybe setup called twice"))?;
    log::set_max_level(log::LevelFilter::Trace);
    Ok(())
}
