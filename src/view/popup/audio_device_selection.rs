use cpal::HostId;
use itertools::Itertools;
use joy_impl_ignore::eq::PartialEqImplIgnore;
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
    audio::device::{Device, Devices},
    keybindings::Action,
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
    devices: Devices,
    selected_list: ListId,
}

impl Default for AudioDeviceSelectionPopup {
    fn default() -> Self {
        Self {
            host_list_state: ListState::default().with_selected(Some(0)),
            device_list_state: ListState::default().with_selected(Some(0)),
            devices: Devices::load(),
            selected_list: ListId::Device,
        }
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

    fn get_current_host_devices(&self) -> Option<&[Device]> {
        self.host_list_state
            .selected()
            .and_then(|list_index| {
                self.devices
                    .hosts()
                    .nth(list_index.clamp(0, self.devices.host_count() - 1))
            })
            .and_then(|host_id| self.devices.devices(&host_id))
    }

    fn selected_device(&self) -> Option<Device> {
        self.device_list_state.selected().and_then(|list_index| {
            self.get_current_host_devices()
                .and_then(|devices| devices.get(list_index.clamp(0, devices.len() - 1)).cloned())
        })
    }

    fn hosts_names(&self) -> Vec<&'static str> {
        self.devices.hosts().map(|id| id.name()).collect_vec()
    }

    fn devices_names(&self) -> Vec<String> {
        self.get_current_host_devices()
            .map(|devices| {
                devices
                    .iter()
                    .map(|Device(name, _)| name.clone())
                    .collect_vec()
            })
            .unwrap_or_default()
    }

    fn popup_widths(&self) -> (u16, u16, u16) {
        let host_list_width = self
            .devices
            .hosts()
            .map(|id: HostId| id.name().len() as u16)
            .max()
            .map(|max| max + Self::LIST_SELECTION_SYMBOL_LEN)
            .unwrap_or(Self::NO_HOST_LABEL_LEN);

        let device_list_width = self
            .get_current_host_devices()
            .and_then(|devices| {
                devices
                    .iter()
                    .map(|Device(name, _)| name.len() as u16)
                    .max()
                    .map(|max| max + Self::LIST_SELECTION_SYMBOL_LEN)
            })
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

        let hosts_names = self.hosts_names();

        if hosts_names.is_empty() {
            Self::NO_HOST_LABEL
                .dark_gray()
                .into_centered_line()
                .render(center_row(area), buf);
        } else {
            StatefulWidget::render(
                List::new(hosts_names)
                    .highlight_spacing(HighlightSpacing::Always)
                    .highlight_symbol(Self::LIST_SELECTION_SYMBOL)
                    .highlight_style(highlight_style),
                area,
                buf,
                &mut self.host_list_state,
            );
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

        let devices_names = self.devices_names();

        if devices_names.is_empty() {
            Self::NO_DEVICE_LABEL
                .dark_gray()
                .into_centered_line()
                .render(center_row(area), buf);
        } else {
            StatefulWidget::render(
                List::new(devices_names)
                    .highlight_spacing(HighlightSpacing::Always)
                    .highlight_symbol(Self::LIST_SELECTION_SYMBOL)
                    .highlight_style(highlight_style),
                area,
                buf,
                &mut self.device_list_state,
            );
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
                    ListId::Host => self.host_list_state.select_previous(),
                    ListId::Device => self.device_list_state.select_previous(),
                },
                Direction::Down => match self.selected_list {
                    ListId::Host => self.host_list_state.select_next(),
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
                    let selected_device: PartialEqImplIgnore<Device> = selected_device.into();
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
