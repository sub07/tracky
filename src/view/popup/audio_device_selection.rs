use std::{collections::HashMap, sync::mpsc::Receiver};

use cpal::HostId;
use itertools::Itertools;
use joy_impl_ignore::eq::PartialEqImplIgnore;
use log::{debug, info};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, ToLine},
    widgets::{Block, BorderType, Clear, ListState, Widget},
};

use crate::{
    audio::device::{self, Device, Host, Hosts},
    event::AsyncEvent,
    keybindings::Action,
    log::DebugLogExt,
    model::Direction,
    view::{center_row, centered_rect, clamp_layout_width},
};

pub enum Popup {
    Loading(Receiver<Hosts>),
    NoHost,
    SelectedHost {
        hosts_name: Vec<String>,
        selected_host_index: usize,
        devices_name: Vec<String>,
    },
}

pub enum Event {
    DataLoaded(Hosts),
    ClosePopup,
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

impl Popup {
    const NO_HOST_LABEL: &str = "There is no availble host on this device";
    const LOADING_LABEL: &str = "Loading...";

    fn render_simple_message(&self, area: Rect, buf: &mut Buffer, message: &str) {
        let area = {
            let message_len = message.len() as u16;
            let area = clamp_layout_width(
                area,
                Constraint::Percentage(60),
                Constraint::Min(message_len + 4),
                Constraint::Max(message_len * 3),
            );

            let [_, area, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Percentage(60),
                Constraint::Fill(1),
            ])
            .areas(area);
            area
        };
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_top(" Device selection");

        let area = {
            let inner = block.inner(area);
            Clear.render(area, buf);
            block.render(area, buf);
            inner
        };

        let message_area = center_row(area);

        message
            .underlined()
            .into_centered_line()
            .render(message_area, buf);
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        match self {
            Popup::Loading(_) => self.render_simple_message(area, buf, Self::LOADING_LABEL),
            Popup::NoHost => self.render_simple_message(area, buf, Self::NO_HOST_LABEL),
            Popup::SelectedHost {
                hosts_name,
                selected_host_index,
                devices_name,
            } => todo!(),
        }
        // match self.view_state {
        //     ViewState::NoHost => todo!(),
        //     ViewState::NoDeviceInHosts { ref host_names } => todo!(),
        //     ViewState::DevicesPresent(ref mut state) => {
        //         let mut device_panel_width = 0;
        //         let popup_width = state.popup_width(&mut device_panel_width);

        //         let area = centered_rect(
        //             area,
        //             Constraint::Min(popup_width),
        //             Constraint::Percentage(50),
        //         );

        //         let block = Block::bordered()
        //             .border_type(BorderType::Rounded)
        //             .title_top("Device selection")
        //             .title_bottom(
        //                 Line::from(DevicePresentState::KEYBINDINGS_LABEL).right_aligned(),
        //             );

        //         let inner_area = {
        //             let inner = block.inner(area);
        //             Clear.render(area, buf);
        //             block.render(area, buf);
        //             inner
        //         };

        //         let [host_panel_area, _, device_panel_area, _] = Layout::horizontal([
        //             Constraint::Length(state.host_panel_width),
        //             Constraint::Length(1),
        //             Constraint::Length(device_panel_width),
        //             Constraint::Fill(1),
        //         ])
        //         .spacing(DevicePresentState::INTER_LIST_GAP)
        //         .flex(Flex::SpaceAround)
        //         .areas(inner_area);

        //         let (host_list_highlight_style, device_list_highlight_style) =
        //             match state.selected_panel {
        //                 Panel::Host => (
        //                     DevicePresentState::SELECTED_HIGHLIGHT_STYLE,
        //                     DevicePresentState::UNSELECTED_HIGHLIGHT_STYLE,
        //                 ),
        //                 Panel::Device => (
        //                     DevicePresentState::UNSELECTED_HIGHLIGHT_STYLE,
        //                     DevicePresentState::SELECTED_HIGHLIGHT_STYLE,
        //                 ),
        //             };

        //         let [host_title_area, host_item_list_area] = Layout::vertical([
        //             Constraint::Length(DevicePresentState::LIST_TITLE_HEIGHT),
        //             Constraint::Fill(1),
        //         ])
        //         .areas(host_panel_area);

        //         DevicePresentState::HOST_LIST_TITLE
        //             .underlined()
        //             .into_centered_line()
        //             .render(host_title_area, buf);

        //         let hosts_names = state.hosts_devices.keys().cloned();

        //         StatefulWidget::render(
        //             List::new(hosts_names)
        //                 .highlight_spacing(HighlightSpacing::Always)
        //                 .highlight_symbol(DevicePresentState::LIST_SELECTION_SYMBOL)
        //                 .highlight_style(host_list_highlight_style),
        //             host_item_list_area,
        //             buf,
        //             &mut state.host_list_state,
        //         );

        //         let [device_title_area, device_item_list_area] = Layout::vertical([
        //             Constraint::Length(DevicePresentState::LIST_TITLE_HEIGHT),
        //             Constraint::Fill(1),
        //         ])
        //         .areas(area);

        //         DevicePresentState::DEVICE_LIST_TITLE
        //             .underlined()
        //             .into_centered_line()
        //             .render(device_title_area, buf);

        //         //     if devices_names.is_empty() {
        //         //         Self::NO_DEVICE_LABEL
        //         //             .dark_gray()
        //         //             .into_centered_line()
        //         //             .render(center_row(area), buf);
        //         //     } else {
        //         //         StatefulWidget::render(
        //         //             List::new(devices_names)
        //         //                 .highlight_spacing(HighlightSpacing::Always)
        //         //                 .highlight_symbol(Self::LIST_SELECTION_SYMBOL)
        //         //                 .highlight_style(highlight_style),
        //         //             area,
        //         //             buf,
        //         //             &mut self.device_list_state,
        //         //         );
        //         //     }
        //     }
        // }
    }

    pub fn map_event(&self, event: &crate::event::Event) -> Option<Event> {
        match event {
            crate::event::Event::App(action) => match action {
                Action::Cancel => Some(Event::ClosePopup),
                _ => None,
            },
            crate::event::Event::Async(async_event) => match async_event {
                AsyncEvent::LoadingDone => match self {
                    Popup::Loading(receiver) => {
                        let hosts = receiver.recv().unwrap();
                        Some(Event::DataLoaded(hosts))
                    }
                    _ => None,
                },
            },
            _ => None,
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Option<crate::event::Event> {
        match event {
            Event::DataLoaded(Hosts(hosts)) => {
                info!("{hosts:?}");
                if hosts.is_empty() {
                    *self = Popup::NoHost;
                } else {
                    *self = Popup::SelectedHost { hosts_name: (), selected_host_index: (), devices_name: () }
                }
                None
            }
            Event::ClosePopup => Some(crate::event::Event::App(Action::ClosePopup)),
        }
    }
}
