use std::iter;

use log::info;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, List, ListState, StatefulWidget, Widget},
};

use crate::{
    audio::{
        device::{sample_format_bit_count, Config, Devices},
        Device, Pan,
    },
    event::{self, Action, EventAware},
    keybindings::InputContext,
    utils::Direction,
    view::{centered_line, theme::THEME},
    EventSender,
};

pub struct State {
    devices: Devices,
    device_list_state: ListState,
    config_list_state: ListState,
    buffer_size_list_state: ListState,
    current_panel: Panel,
}

pub enum Event {
    SelectNextDevice,
    SelectPreviousDevice,

    SelectNextConfig,
    SelectPreviousConfig,

    SelectNextBufferSize,
    SelectPreviousBufferSize,

    SetPanel(Panel),

    PickDevice,
}

enum Panel {
    Device,
    Config,
    BufferSize,
}

impl From<Devices> for State {
    fn from(devices: Devices) -> Self {
        Self::new(devices)
    }
}

impl EventAware<Event> for State {
    fn map_event(&self, event: &event::Event) -> Option<Event> {
        match (&self.current_panel, event) {
            (Panel::Device, event::Event::Action(Action::Move(Direction::Down))) => {
                Some(Event::SelectNextDevice)
            }
            (Panel::Device, event::Event::Action(Action::Move(Direction::Up))) => {
                Some(Event::SelectPreviousDevice)
            }

            (Panel::Config, event::Event::Action(Action::Move(Direction::Down))) => {
                Some(Event::SelectNextConfig)
            }
            (Panel::Config, event::Event::Action(Action::Move(Direction::Up))) => {
                Some(Event::SelectPreviousConfig)
            }

            (Panel::BufferSize, event::Event::Action(Action::Move(Direction::Down))) => {
                Some(Event::SelectNextBufferSize)
            }
            (Panel::BufferSize, event::Event::Action(Action::Move(Direction::Up))) => {
                Some(Event::SelectPreviousBufferSize)
            }

            (Panel::Device, event::Event::Action(Action::Move(Direction::Right))) => {
                Some(Event::SetPanel(Panel::Config))
            }

            (Panel::Config, event::Event::Action(Action::Move(Direction::Left))) => {
                Some(Event::SetPanel(Panel::Device))
            }
            (Panel::Config, event::Event::Action(Action::Move(Direction::Right))) => {
                Some(Event::SetPanel(Panel::BufferSize))
            }

            (Panel::BufferSize, event::Event::Action(Action::Move(Direction::Left))) => {
                Some(Event::SetPanel(Panel::Config))
            }

            (Panel::BufferSize, event::Event::Action(Action::Confirm)) => Some(Event::PickDevice),
            _ => None,
        }
    }

    fn update(&mut self, event: Event, event_tx: EventSender) {
        match event {
            Event::SelectNextDevice => {
                self.device_list_state.select_next();
                self.config_list_state = ListState::default();
                self.buffer_size_list_state = ListState::default();
            }
            Event::SelectPreviousDevice => {
                self.device_list_state.select_previous();
                self.config_list_state = ListState::default();
                self.buffer_size_list_state = ListState::default();
            }
            Event::SelectNextConfig => {
                self.config_list_state.select_next();
                self.buffer_size_list_state = ListState::default();
            }
            Event::SelectPreviousConfig => {
                self.config_list_state.select_previous();
                self.buffer_size_list_state = ListState::default();
            }
            Event::SelectNextBufferSize => self.buffer_size_list_state.select_next(),
            Event::SelectPreviousBufferSize => self.buffer_size_list_state.select_previous(),
            Event::SetPanel(panel) => self.set_panel(panel),
            Event::PickDevice => {
                if let (device, Some((config_index, buffer_size_index))) = (
                    self.get_selected_device(),
                    self.config_list_state
                        .selected()
                        .zip(self.buffer_size_list_state.selected()),
                ) {
                    let config = self.get_selected_config().unwrap();
                    let buffer_size = match buffer_size_index {
                        0 => cpal::BufferSize::Default,
                        index => cpal::BufferSize::Fixed(config.buffer_sizes.unwrap()[index - 1]),
                    };
                    let device = device.configure(buffer_size, config_index);

                    event_tx
                        .send_event(event::Event::Composite(vec![
                            event::Event::SetPlayingDevice(dbg!(device)),
                            event::Event::StartAudioPlayer,
                        ]))
                        .unwrap();
                }
            }
        }
    }

    fn input_context(&self) -> InputContext {
        InputContext::Global
    }
}

impl State {
    const SELECTED_PANEL_HIGHLIGHT: Style =
        Style::new().bg(THEME.cursor_background).fg(THEME.on_cursor);
    const UNSELECTED_PANEL_HIGHLIGHT: Style = Style::new()
        .bg(THEME.elevated_background)
        .fg(THEME.on_elevated_background);

    fn new(devices: Devices) -> Self {
        let mut state = Self {
            devices: dbg!(devices),
            device_list_state: ListState::default(),
            config_list_state: ListState::default(),
            buffer_size_list_state: ListState::default(),
            current_panel: Panel::Device,
        };

        state.set_panel(Panel::Device);

        state
    }

    fn set_panel(&mut self, panel: Panel) {
        match panel {
            Panel::Device => {
                if self.device_list_state.selected().is_none() {
                    self.device_list_state.select_first();
                }
                self.config_list_state = ListState::default();
                self.buffer_size_list_state = ListState::default();
            }
            Panel::Config => {
                if self.config_list_state.selected().is_none() {
                    self.config_list_state.select_first();
                }
                self.buffer_size_list_state = ListState::default();
            }
            Panel::BufferSize => {
                if self.buffer_size_list_state.selected().is_none() {
                    self.buffer_size_list_state.select_first();
                }
            }
        }
        self.current_panel = panel;
    }

    fn get_selected_device(&self) -> &Device {
        &self.devices.0[self.device_list_state.selected().unwrap()]
    }

    fn get_selected_config(&self) -> Option<&Config> {
        self.config_list_state
            .selected()
            .map(|index| &self.get_selected_device().configs[index])
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [device_list_area, device_config_area, buffer_size_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        let (
            device_list_highlight_style,
            option_list_highlight_style,
            buffer_size_list_highlight_style,
        ) = match self.current_panel {
            Panel::Device => (
                Self::SELECTED_PANEL_HIGHLIGHT,
                Self::UNSELECTED_PANEL_HIGHLIGHT,
                Self::UNSELECTED_PANEL_HIGHLIGHT,
            ),
            Panel::Config => (
                Self::UNSELECTED_PANEL_HIGHLIGHT,
                Self::SELECTED_PANEL_HIGHLIGHT,
                Self::UNSELECTED_PANEL_HIGHLIGHT,
            ),
            Panel::BufferSize => (
                Self::UNSELECTED_PANEL_HIGHLIGHT,
                Self::UNSELECTED_PANEL_HIGHLIGHT,
                Self::SELECTED_PANEL_HIGHLIGHT,
            ),
        };

        StatefulWidget::render(
            List::new(
                self.devices
                    .0
                    .iter()
                    .map(|device| format!("[{}] {}", device.host_name, device.name)),
            )
            .block(Block::bordered())
            .highlight_style(device_list_highlight_style),
            device_list_area,
            buf,
            &mut self.device_list_state,
        );

        StatefulWidget::render(
            List::new(self.get_selected_device().configs.iter().map(|config| {
                format!(
                    "{}Hz - {}bits({})",
                    config.sample_rate,
                    sample_format_bit_count(config.sample_format),
                    config.sample_format
                )
            }))
            .highlight_style(option_list_highlight_style)
            .block(Block::bordered()),
            device_config_area,
            buf,
            &mut self.config_list_state,
        );

        if let Some(config) = self.get_selected_config() {
            let buffer_size_list_items: &mut dyn Iterator<Item = String> =
                if let Some(buffer_sizes) = config.buffer_sizes {
                    &mut iter::once("Default".to_owned()).chain(
                        buffer_sizes
                            .iter()
                            .map(|buffer_size| format!("{buffer_size} samples")),
                    )
                } else {
                    &mut iter::once("Default".to_owned())
                };

            StatefulWidget::render(
                List::new(buffer_size_list_items)
                    .highlight_style(buffer_size_list_highlight_style)
                    .block(Block::bordered()),
                buffer_size_area,
                buf,
                &mut self.buffer_size_list_state,
            );
        } else {
            Widget::render(Block::bordered(), buffer_size_area, buf);
            Widget::render(
                Line::raw("Select a config").centered(),
                centered_line(buffer_size_area),
                buf,
            );
        }
    }
}
