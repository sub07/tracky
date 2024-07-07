use std::collections::HashMap;

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Host, HostId, ALL_HOSTS,
};
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    prelude::Style,
    style::{Color, Stylize},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Clear, HighlightSpacing, List, ListState, StatefulWidget, Widget,
    },
};

use crate::{
    keybindings::Action,
    log::DebugLogExt,
    model::Direction,
    view::{center_row, centered_rect},
};

enum ListId {
    Host,
    Device,
}

pub struct AudioDeviceSelectionPopup {
    host_list_state: ListState,
    device_list_state: ListState,
    hosts: HashMap<HostId, Host>,
    devices: Vec<(String, Device)>,
    selected_list: ListId,
}

impl Default for AudioDeviceSelectionPopup {
    fn default() -> Self {
        let hosts = ALL_HOSTS
            .iter()
            .filter_map(|id| cpal::host_from_id(*id).ok().map(|host| (*id, host)))
            .collect();

        let host_list_state = ListState::default().with_selected(Some(0));
        let device_list_state = ListState::default().with_selected(Some(0));

        let mut state = Self {
            host_list_state,
            device_list_state,
            hosts,
            devices: Vec::new(),
            selected_list: ListId::Device,
        };

        state.load_selected_host_devices();

        state
    }
}

impl AudioDeviceSelectionPopup {
    const LIST_GAP: u16 = 1;
    const LIST_SPACING: u16 = 1;
    const NO_HOST_LABEL: &str = "No available host";
    const NO_HOST_LABEL_LEN: u16 = Self::NO_HOST_LABEL.len() as u16;
    const NO_DEVICE_LABEL: &str = "No available device from this host";
    const NO_DEVICE_LABEL_LEN: u16 = Self::NO_DEVICE_LABEL.len() as u16;
    const LIST_SELECTION_SYMBOL: &str = "\u{2B9E} "; // ⮞
    const LIST_SELECTION_SYMBOL_LEN: u16 = Self::LIST_SELECTION_SYMBOL.len() as u16;
    const UNSELECTED_HIGHLIGHT_STYLE: Style = Style::new().fg(Color::White).bg(Color::Black);
    const SELECTED_HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Black).bg(Color::White);
    const HOST_LIST_TITLE: &str = "Hosts";
    const DEVICE_LIST_TITLE: &str = "Devices";
    const LIST_TITLE_HEIGHT: u16 = 2;
    const KEYBINDINGS_LABEL: &str = " <Tab> Shift - <Esc> Cancel - <Enter> Confirm ";
    const KEYBINDINGS_LABEL_LEN: u16 = Self::KEYBINDINGS_LABEL.len() as u16;

    fn selected_host_id(&self) -> Option<HostId> {
        self.host_list_state.selected().and_then(|list_index| {
            self.hosts
                .keys()
                .nth(list_index.clamp(0, self.hosts.len() - 1))
                .cloned()
        })
    }

    fn load_host_devices(&mut self, host_id: HostId) {
        let devices = self.hosts[&host_id].output_devices().map(|devices| {
            devices
                .map(|device| {
                    (
                        device
                            .name()
                            .unwrap_or("Output (error while getting name)".into()),
                        device,
                    )
                })
                .collect::<Vec<_>>()
        });

        match devices {
            Ok(devices) => self.devices = devices,
            Err(err) => err.error("device loading"),
        }
    }

    fn load_selected_host_devices(&mut self) {
        if let Some(host) = self.selected_host_id() {
            self.load_host_devices(host);
        }
    }

    fn selected_device(&self) -> Option<Device> {
        self.device_list_state.selected().and_then(|list_index| {
            self.devices
                .get(list_index.clamp(0, self.devices.len() - 1))
                .map(|(_, device)| device.clone())
        })
    }

    fn hosts(&self) -> Option<impl Iterator<Item = &'static str> + '_> {
        if self.hosts.is_empty() {
            None
        } else {
            Some(self.hosts.keys().map(|id| id.name()))
        }
    }

    fn devices(&self) -> Option<impl Iterator<Item = String> + '_> {
        if self.devices.is_empty() {
            None
        } else {
            Some(self.devices.iter().map(|(name, _)| name.to_owned()))
        }
    }

    fn popup_widths(&self) -> (u16, u16, u16) {
        let host_list_width = self
            .hosts
            .keys()
            .map(|id: &HostId| id.name().len() as u16)
            .max()
            .map(|max| max + Self::LIST_SELECTION_SYMBOL_LEN)
            .unwrap_or(Self::NO_HOST_LABEL_LEN);

        let device_list_width = self
            .devices
            .iter()
            .map(|(name, _)| name.len() as u16)
            .max()
            .map(|max| max + Self::LIST_SELECTION_SYMBOL_LEN)
            .unwrap_or(Self::NO_DEVICE_LABEL_LEN);

        let inner_width = u16::max(
            Self::LIST_SPACING
                + host_list_width
                + Self::LIST_SPACING
                + Self::LIST_GAP
                + Self::LIST_SPACING
                + device_list_width
                + Self::LIST_SPACING,
            Self::KEYBINDINGS_LABEL_LEN,
        );

        (
            // | *----   *---- |
            1 + inner_width + 1,
            host_list_width,
            device_list_width,
        )
    }

    pub fn render_host_list(&mut self, highlight_style: Style, area: Rect, buf: &mut Buffer) {
        let [title_area, area] = Layout::vertical([
            Constraint::Length(Self::LIST_TITLE_HEIGHT),
            Constraint::Fill(1),
        ])
        .areas(area);

        Self::HOST_LIST_TITLE
            .underlined()
            .into_centered_line()
            .render(title_area, buf);

        if let Some(hosts) = self.hosts().map(|iter| iter.collect_vec()) {
            StatefulWidget::render(
                List::new(hosts)
                    .highlight_spacing(HighlightSpacing::Always)
                    .highlight_symbol(Self::LIST_SELECTION_SYMBOL)
                    .highlight_style(highlight_style),
                area,
                buf,
                &mut self.host_list_state,
            );
        } else {
            Self::NO_HOST_LABEL
                .dark_gray()
                .into_centered_line()
                .render(center_row(area), buf);
        }
    }

    pub fn render_device_list(&mut self, highlight_style: Style, area: Rect, buf: &mut Buffer) {
        let [title_area, area] = Layout::vertical([
            Constraint::Length(Self::LIST_TITLE_HEIGHT),
            Constraint::Fill(1),
        ])
        .areas(area);

        Self::DEVICE_LIST_TITLE
            .underlined()
            .into_centered_line()
            .render(title_area, buf);

        if let Some(devices) = self.devices().map(|iter| iter.collect_vec()) {
            StatefulWidget::render(
                List::new(devices)
                    .highlight_spacing(HighlightSpacing::Always)
                    .highlight_symbol(Self::LIST_SELECTION_SYMBOL)
                    .highlight_style(highlight_style),
                area,
                buf,
                &mut self.device_list_state,
            );
        } else {
            Self::NO_DEVICE_LABEL
                .dark_gray()
                .into_centered_line()
                .render(center_row(area), buf);
        }
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let (popup_width, host_list_width, device_list_width) = self.popup_widths();
        let area = centered_rect(
            area,
            Constraint::Min(popup_width),
            Constraint::Percentage(50),
        );
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(Title::from(" Device selection ").alignment(Alignment::Left))
            .title(
                Title::from(Self::KEYBINDINGS_LABEL)
                    .position(Position::Bottom)
                    .alignment(Alignment::Right),
            );

        let area = {
            let inner = block.inner(area);
            Clear.render(area, buf);
            block.render(area, buf);
            inner
        };

        let [host_list_area, _, device_list_area] = Layout::horizontal([
            Constraint::Min(host_list_width),
            Constraint::Length(Self::LIST_GAP),
            Constraint::Min(device_list_width),
        ])
        .spacing(Self::LIST_SPACING)
        .flex(Flex::SpaceAround)
        .areas(area);

        let (host_list_highlight_style, device_list_highlight_style) = match self.selected_list {
            ListId::Host => (
                Self::SELECTED_HIGHLIGHT_STYLE,
                Self::UNSELECTED_HIGHLIGHT_STYLE,
            ),
            ListId::Device => (
                Self::UNSELECTED_HIGHLIGHT_STYLE,
                Self::SELECTED_HIGHLIGHT_STYLE,
            ),
        };

        self.render_host_list(host_list_highlight_style, host_list_area, buf);
        self.render_device_list(device_list_highlight_style, device_list_area, buf);
    }

    pub fn handle_action(&mut self, action: Action, consumed: &mut bool) -> Option<Action> {
        match action {
            Action::Move(direction, _) => match direction {
                Direction::Up => match self.selected_list {
                    ListId::Host => {
                        self.host_list_state.select_previous();
                        self.load_selected_host_devices();
                    }
                    ListId::Device => self.device_list_state.select_previous(),
                },
                Direction::Down => match self.selected_list {
                    ListId::Host => {
                        self.host_list_state.select_next();
                        self.load_selected_host_devices();
                    }
                    ListId::Device => self.device_list_state.select_next(),
                },
                _ => {
                    self.selected_list = match self.selected_list {
                        ListId::Host => ListId::Device,
                        ListId::Device => ListId::Host,
                    }
                }
            },
            Action::Confirm => {
                if let Some(selected_device) = self.selected_device() {
                    return Some(Action::Composite(vec![
                        Action::SetPlayingDevice(selected_device.into()),
                        Action::ClosePopup,
                    ]));
                }
            }
            Action::Cancel => return Some(Action::ClosePopup),
            _ => *consumed = false,
        }
        None
    }
}
