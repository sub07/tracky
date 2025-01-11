use std::{
    fmt::{Debug, Display},
    fs, iter,
    ops::{Deref, DerefMut},
    path::Path,
    sync::Mutex,
};

use anyhow::{anyhow, Context};
use itertools::Itertools;

use joy_macro::{EnumIter, EnumStr};
use log::debug;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Text, ToLine},
    widgets::{Block, BorderType, Clear, Paragraph},
    Frame,
};

#[derive(Clone, Copy, Debug, PartialEq, EnumStr, EnumIter)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

#[derive(Clone, PartialEq)]
struct SingleLogEntry {
    content: String,
    level: LogLevel,
    line_count: usize,
}

#[derive(Clone)]
enum LogEntry {
    Single(SingleLogEntry),
    Multiple(usize, SingleLogEntry),
}

impl LogEntry {
    fn inner(&self) -> &SingleLogEntry {
        match self {
            LogEntry::Single(single_log_entry) => single_log_entry,
            LogEntry::Multiple(_, single_log_entry) => single_log_entry,
        }
    }

    fn inner_mut(&mut self) -> &mut SingleLogEntry {
        match self {
            LogEntry::Single(single_log_entry) => single_log_entry,
            LogEntry::Multiple(_, single_log_entry) => single_log_entry,
        }
    }

    fn count(&self) -> Option<usize> {
        match self {
            LogEntry::Single(_) => None,
            LogEntry::Multiple(count, _) => Some(*count),
        }
    }
}

#[derive(Default, Clone)]
struct TerminalLogger {
    entries: Vec<LogEntry>,
}

static TERMINAL_LOGGER: Mutex<TerminalLogger> = Mutex::new(TerminalLogger {
    entries: Vec::new(),
});

pub fn write_logs_to_file<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let logger = read_entries(|logger| logger.clone());
    let logs = logger
        .entries
        .into_iter()
        .map(|entry| match entry {
            LogEntry::Single(single_log_entry) => {
                Box::new(iter::once(single_log_entry)) as Box<dyn Iterator<Item = SingleLogEntry>>
            }
            LogEntry::Multiple(count, single_log_entry) => {
                Box::new(iter::repeat_n(single_log_entry, count))
            }
        })
        .flatten()
        .map(|entry| format!("{:?} - {}", entry.level, entry.content))
        .join("\n");

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
        let single_entry = SingleLogEntry {
            content,
            level,
            line_count,
        };
        let entry_count = logger.entries.len();
        match logger.entries.last() {
            Some(LogEntry::Single(last_single_entry)) if last_single_entry == &single_entry => {
                logger.entries[entry_count - 1] = LogEntry::Multiple(2, single_entry);
            }
            Some(LogEntry::Multiple(count, last_single_entry))
                if last_single_entry == &single_entry =>
            {
                logger.entries[entry_count - 1] = LogEntry::Multiple(count + 1, single_entry);
            }
            _ => {
                logger.entries.push(LogEntry::Single(single_entry));
            }
        }
    })
}

pub trait DebugLogExt {
    #[allow(dead_code, reason = "dev utils")]
    fn debug(self, tag: &'static str) -> Self;
}

impl<T: Debug> DebugLogExt for T {
    fn debug(self, tag: &'static str) -> Self {
        debug!("{tag}: {:#?}", &self);
        self
    }
}

pub fn clear_entries() {
    alter_entries(|logger| logger.entries.clear());
}

fn build_block_widget() -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title_bottom(
            " <F9> Save on Disk - <F10> Clear - <F12> Toggle "
                .to_line()
                .right_aligned(),
        )
}

#[allow(unstable_name_collisions)]
pub fn render_log_panel(frame: &mut Frame, area: Rect) {
    let block = build_block_widget();

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
                    line_count += entry.inner().line_count;
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
        oldest_entry.inner_mut().content = oldest_entry
            .inner()
            .content
            .lines()
            .skip(line_count - available_line_count)
            .intersperse("\n")
            .collect::<String>()
    }

    const MAX_LOG_LEVEL_TEXT_LEN: usize = const {
        let mut max = usize::MIN;
        let mut i = 0;

        while i < LogLevel::COUNT {
            let log_label_len = LogLevel::VARIANTS[i].as_str().len();
            if max < log_label_len {
                max = log_label_len;
            }
            i += 1;
        }
        max
    };

    let text = entries
        .iter()
        .rev()
        .flat_map(|entry| {
            entry
                .inner()
                .content
                .lines()
                .enumerate()
                .map(|(line_index, line)| {
                    if line_index == 0 {
                        let tag_color = match entry.inner().level {
                            LogLevel::Error => Color::Red,
                            LogLevel::Info => Color::LightBlue,
                            LogLevel::Debug => Color::DarkGray,
                            LogLevel::Warn => Color::Yellow,
                        };
                        Line::from_iter([
                            if let Some(count) = entry.count() {
                                count.to_string()
                            } else {
                                String::new()
                            }
                            .fg(Color::DarkGray),
                            format!(
                                "{:>width$} ",
                                entry.inner().level.as_str(),
                                width = MAX_LOG_LEVEL_TEXT_LEN
                            )
                            .fg(tag_color)
                            .italic(),
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
    let block = build_block_widget();

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
