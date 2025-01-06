use std::collections::HashMap;

use cpal::HostId;
use itertools::Itertools;
use joy_impl_ignore::eq::PartialEqImplIgnore;
use log::{debug, info};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    prelude::Style,
    style::{Color, Stylize},
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Clear, HighlightSpacing, List, ListState, StatefulWidget, Widget,
    },
};

use crate::{
    audio::device::{self, Device, Host, Hosts},
    keybindings::Action,
    model::Direction,
    view::{center_row, centered_rect},
};

pub struct AudioDeviceSelectionPopup {
    view_state: ViewState,
}

impl Default for AudioDeviceSelectionPopup {
    fn default() -> Self {
        let Hosts(mut hosts) = Hosts::load();
        let view_state = if hosts.is_empty() {
            ViewState::NoHost
        } else if hosts.iter().all(|host| host.devices.is_empty()) {
            ViewState::NoDeviceInHosts {
                host_names: hosts.into_iter().map(|host| host.name).collect(),
            }
        } else {
            hosts.sort_by(|h1, h2| h2.devices.len().cmp(&h1.devices.len()));
            ViewState::DevicesPresent(DevicePresentState::new(
                ListState::default().with_selected(Some(0)),
                ListState::default().with_selected(Some(0)),
                hosts
                    .into_iter()
                    .filter(|host| !host.devices.is_empty())
                    .map(|host| (host.name, host.devices))
                    .collect(),
                Panel::Host,
            ))
        };
        Self { view_state }
    }
}

enum ViewState {
    NoHost,
    NoDeviceInHosts { host_names: Vec<String> },
    DevicesPresent(DevicePresentState),
}

struct DevicePresentState {
    host_list_state: ListState,
    device_list_state: ListState,
    hosts_devices: HashMap<String, Vec<Device>>,
    selected_panel: Panel,
    host_panel_width: u16,
}

impl DevicePresentState {
    const INTER_LIST_GAP: u16 = 1;
    const LIST_SPACING: u16 = 1;
    const NO_DEVICE_LABEL: &str = "No available device from this host";
    const NO_DEVICE_LABEL_LEN: u16 = Self::NO_DEVICE_LABEL.len() as u16;
    const LIST_SELECTION_SYMBOL: &str = "\u{2B9E} "; // ⮞
    const UNSELECTED_HIGHLIGHT_STYLE: Style = Style::new().fg(Color::White).bg(Color::Black);
    const SELECTED_HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Black).bg(Color::White);
    const HOST_LIST_TITLE: &str = "Hosts";
    const DEVICE_LIST_TITLE: &str = "Devices";
    const LIST_TITLE_HEIGHT: u16 = 2;
    const KEYBINDINGS_LABEL: &str = " <Tab> Shift - <Esc> Cancel - <Enter> Confirm ";
    const LIST_SELECTION_SYMBOL_LEN: u16 = Self::LIST_SELECTION_SYMBOL.len() as u16;

    fn new(
        host_list_state: ListState,
        device_list_state: ListState,
        hosts_devices: HashMap<String, Vec<Device>>,
        selected_panel: Panel,
    ) -> Self {
        let host_panel_width = hosts_devices
            .keys()
            .map(|host_name| host_name.len())
            .max()
            .expect("Map should not be empty in this state") as u16
            + Self::LIST_SELECTION_SYMBOL_LEN;

        let host_panel_width = host_panel_width.max(const { Self::HOST_LIST_TITLE.len() as u16 });
        Self {
            host_list_state,
            device_list_state,
            hosts_devices,
            selected_panel,
            host_panel_width,
        }
    }

    fn selected_host(&self) -> String {
        self.host_list_state
            .selected()
            .and_then(|host_index| self.hosts_devices.keys().nth(host_index))
            .unwrap()
            .clone()
    }

    fn device_panel_width(&self) -> u16 {
        self.hosts_devices[&self.selected_host()]
            .iter()
            .map(|device| device.name.len())
            .max()
            .unwrap() as u16
            + Self::LIST_SELECTION_SYMBOL_LEN
    }

    fn popup_width(&self, out_device_panel_width: &mut u16) -> u16 {
        *out_device_panel_width = self.device_panel_width();
        debug!("{}", self.host_panel_width);
        debug!("{}", *out_device_panel_width);
        let popup_width = u16::max(
            Self::LIST_SPACING
                + self.host_panel_width
                + Self::LIST_SPACING
                + Self::INTER_LIST_GAP
                + Self::LIST_SPACING
                + *out_device_panel_width
                + Self::LIST_SPACING,
            const { Self::KEYBINDINGS_LABEL.len() as u16 } + 2,
        );

        1 + popup_width + 1
    }
}

enum Panel {
    Host,
    Device,
}

impl AudioDeviceSelectionPopup {
    const NO_HOST_LABEL: &str = "No available host";

    // pub fn render_host_list(&mut self, highlight_style: Style, area: Rect, buf: &mut Buffer) {
    //
    // }

    // pub fn render_device_list(&mut self, highlight_style: Style, area: Rect, buf: &mut Buffer) {
    //
    // }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        match self.view_state {
            ViewState::NoHost => todo!(),
            ViewState::NoDeviceInHosts { ref host_names } => todo!(),
            ViewState::DevicesPresent(ref mut state) => {
                let mut device_panel_width = 0;
                let popup_width = state.popup_width(&mut device_panel_width);

                let area = centered_rect(
                    area,
                    Constraint::Min(popup_width),
                    Constraint::Percentage(50),
                );

                let block = Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top("Device selection")
                    .title_bottom(
                        Line::from(DevicePresentState::KEYBINDINGS_LABEL).right_aligned(),
                    );

                let inner_area = {
                    let inner = block.inner(area);
                    Clear.render(area, buf);
                    block.render(area, buf);
                    inner
                };

                let [host_panel_area, _, device_panel_area, _] = Layout::horizontal([
                    Constraint::Length(state.host_panel_width),
                    Constraint::Length(1),
                    Constraint::Length(device_panel_width),
                    Constraint::Fill(1),
                ])
                .spacing(DevicePresentState::INTER_LIST_GAP)
                .flex(Flex::SpaceAround)
                .areas(inner_area);

                let (host_list_highlight_style, device_list_highlight_style) =
                    match state.selected_panel {
                        Panel::Host => (
                            DevicePresentState::SELECTED_HIGHLIGHT_STYLE,
                            DevicePresentState::UNSELECTED_HIGHLIGHT_STYLE,
                        ),
                        Panel::Device => (
                            DevicePresentState::UNSELECTED_HIGHLIGHT_STYLE,
                            DevicePresentState::SELECTED_HIGHLIGHT_STYLE,
                        ),
                    };

                let [host_title_area, host_item_list_area] = Layout::vertical([
                    Constraint::Length(DevicePresentState::LIST_TITLE_HEIGHT),
                    Constraint::Fill(1),
                ])
                .areas(host_panel_area);

                DevicePresentState::HOST_LIST_TITLE
                    .underlined()
                    .into_centered_line()
                    .render(host_title_area, buf);

                let hosts_names = state.hosts_devices.keys().cloned();

                StatefulWidget::render(
                    List::new(hosts_names)
                        .highlight_spacing(HighlightSpacing::Always)
                        .highlight_symbol(DevicePresentState::LIST_SELECTION_SYMBOL)
                        .highlight_style(host_list_highlight_style),
                    host_item_list_area,
                    buf,
                    &mut state.host_list_state,
                );

                let [device_title_area, device_item_list_area] = Layout::vertical([
                    Constraint::Length(DevicePresentState::LIST_TITLE_HEIGHT),
                    Constraint::Fill(1),
                ])
                .areas(area);

                DevicePresentState::DEVICE_LIST_TITLE
                    .underlined()
                    .into_centered_line()
                    .render(device_title_area, buf);
                
                

                //     if devices_names.is_empty() {
                //         Self::NO_DEVICE_LABEL
                //             .dark_gray()
                //             .into_centered_line()
                //             .render(center_row(area), buf);
                //     } else {
                //         StatefulWidget::render(
                //             List::new(devices_names)
                //                 .highlight_spacing(HighlightSpacing::Always)
                //                 .highlight_symbol(Self::LIST_SELECTION_SYMBOL)
                //                 .highlight_style(highlight_style),
                //             area,
                //             buf,
                //             &mut self.device_list_state,
                //         );
                //     }
            }
        }
    }

    pub fn handle_action(&mut self, action: Action, consumed: &mut bool) -> Option<Action> {
        match action {
            //     Action::Move(direction, _) => match direction {
            //         Direction::Up => match self.selected_list {
            //             Panel::Host => self.host_list_state.select_previous(),
            //             Panel::Device => self.device_list_state.select_previous(),
            //         },
            //         Direction::Down => match self.selected_list {
            //             Panel::Host => self.host_list_state.select_next(),
            //             Panel::Device => self.device_list_state.select_next(),
            //         },
            //         _ => {
            //             self.selected_list = match self.selected_list {
            //                 Panel::Host => Panel::Device,
            //                 Panel::Device => Panel::Host,
            //             }
            //         }
            //     },
            //     Action::Confirm => {
            //         if let Some(selected_device) = self.selected_device() {
            //             let selected_device: PartialEqImplIgnore<Device> = selected_device.into();
            //             return Some(Action::Composite(vec![
            //                 Action::SetPlayingDevice(selected_device.into()),
            //                 Action::ClosePopup,
            //             ]));
            //         }
            //     }
            Action::Cancel => return Some(Action::ClosePopup),
            _ => *consumed = false,
        }
        None
    }
}
