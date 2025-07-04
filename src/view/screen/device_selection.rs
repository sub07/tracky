use std::iter;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, List, ListState, StatefulWidget, Widget},
};

use crate::{
    audio::{
        device::{sample_format_bit_count, Config, Devices},
        Device,
    },
    event::{self, Action, HandleAction},
    keybindings::InputContext,
    utils::Direction,
    view::{centered_line, theme::THEME},
    EventSender,
};

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Panel {
    Device,
    Config,
    BufferSize,
}

impl From<Devices> for State {
    fn from(devices: Devices) -> Self {
        Self::new(devices)
    }
}

impl HandleAction<Event> for State {
    fn map_action(&self, action: &Action) -> Option<Event> {
        match (&self.current_panel, action) {
            (Panel::Device, Action::Move(Direction::Down)) => Some(Event::SelectNextDevice),
            (Panel::Device, Action::Move(Direction::Up)) => Some(Event::SelectPreviousDevice),
            (Panel::Config, Action::Move(Direction::Down)) => Some(Event::SelectNextConfig),
            (Panel::Config, Action::Move(Direction::Up)) => Some(Event::SelectPreviousConfig),
            (Panel::BufferSize, Action::Move(Direction::Down)) => Some(Event::SelectNextBufferSize),
            (Panel::BufferSize, Action::Move(Direction::Up)) => {
                Some(Event::SelectPreviousBufferSize)
            }
            (Panel::Device, Action::Move(Direction::Right)) => Some(Event::SetPanel(Panel::Config)),
            (Panel::Config, Action::Move(Direction::Left)) => Some(Event::SetPanel(Panel::Device)),
            (Panel::Config, Action::Move(Direction::Right)) => {
                Some(Event::SetPanel(Panel::BufferSize))
            }
            (Panel::BufferSize, Action::Move(Direction::Left)) => {
                Some(Event::SetPanel(Panel::Config))
            }
            (Panel::BufferSize, Action::Confirm) => Some(Event::PickDevice),
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
                THEME.primary_cursor,
                THEME.secondary_cursor,
                THEME.secondary_cursor,
            ),
            Panel::Config => (
                THEME.secondary_cursor,
                THEME.primary_cursor,
                THEME.secondary_cursor,
            ),
            Panel::BufferSize => (
                THEME.secondary_cursor,
                THEME.secondary_cursor,
                THEME.primary_cursor,
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
