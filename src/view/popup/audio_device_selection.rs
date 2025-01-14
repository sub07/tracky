use std::sync::mpsc::Sender;

use itertools::Itertools;
use joy_vector::Vector;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, ToLine},
    widgets::{
        Block, BorderType, Clear, HighlightSpacing, List, ListState, StatefulWidget, Widget,
    },
};

use crate::{
    audio::device::{Device, Hosts},
    event::{Action, Event},
    model::Direction,
    view::{centered_line, responsive_centered_rect},
};

pub enum Popup {
    NoHost,
    SelectedHost(SelectedHostState),
}

impl From<Hosts> for Popup {
    fn from(value: Hosts) -> Self {
        if value.0.is_empty() {
            Popup::NoHost
        } else {
            Popup::SelectedHost(SelectedHostState::new(value))
        }
    }
}

pub enum PopupEvent {
    ClosePopup,
    Move(Direction),
    Select,
}

enum Panel {
    Host,
    Device,
}

pub struct SelectedHostState {
    hosts_name: Vec<String>,
    selected_host_index: usize,
    devices: Vec<Device>,
    cache: Hosts,
    device_list_state: ListState,
    host_panel_width: u16,
    device_panel_width: u16,
    popup_width: u16,
    selected_panel: Panel,
}

impl SelectedHostState {
    const LIST_SELECTION_SYMBOL: &str = "\u{2B9E} "; // ⮞
    const LIST_SELECTION_SYMBOL_LEN: u16 = Self::LIST_SELECTION_SYMBOL.len() as u16;

    const HOST_LIST_TITLE: &str = "Hosts";
    const DEVICE_LIST_TITLE: &str = "Devices";
    const LIST_TITLE_HEIGHT: u16 = 2;

    const LIST_SPACING: u16 = 1;
    const INTER_LIST_GAP: u16 = 1;

    const KEYBINDINGS_LABEL: &str = " <Tab> Shift - <Esc> Cancel - <Enter> Confirm ";

    const UNSELECTED_HIGHLIGHT_STYLE: Style = Style::new().fg(Color::White).bg(Color::Black);
    const SELECTED_HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Black).bg(Color::White);

    pub fn new(cache: Hosts) -> Self {
        let hosts_name = cache.0.iter().map(|host| host.name.clone()).collect_vec();

        let host_panel_width = hosts_name
            .iter()
            .map(String::len)
            .max()
            .map(|width| width as u16 + SelectedHostState::LIST_SELECTION_SYMBOL_LEN)
            .unwrap()
            .max(const { Self::HOST_LIST_TITLE.len() as u16 });

        let selected_panel = if hosts_name.len() == 1 {
            Panel::Device
        } else {
            Panel::Host
        };

        let device_list_state = ListState::default().with_selected(Some(0));

        let mut state = Self {
            hosts_name,
            selected_host_index: 0,
            devices: Vec::new(),
            cache,
            device_list_state,
            host_panel_width,
            device_panel_width: 0,
            popup_width: 0,
            selected_panel,
        };
        state.load_device_from_selected_host();

        state
    }

    fn move_cursor(&mut self, direction: Direction) {
        match (direction.vector(), &self.selected_panel) {
            (Vector([0, d]), Panel::Host) => {
                let mut selected_host_index = self.selected_host_index as i32;
                selected_host_index += d;
                selected_host_index = selected_host_index.rem_euclid(self.hosts_name.len() as i32);
                self.selected_host_index = selected_host_index as usize;
                self.load_device_from_selected_host();
            }
            (Vector([0, d]), Panel::Device) => {
                let mut selected_device_index = self.device_list_state.selected().unwrap() as i32;
                selected_device_index += d;
                selected_device_index = selected_device_index.rem_euclid(self.devices.len() as i32);
                self.device_list_state
                    .select(Some(selected_device_index as usize));
            }
            (Vector([_, 0]), _) => {
                self.selected_panel = if matches!(self.selected_panel, Panel::Device) {
                    Panel::Host
                } else {
                    Panel::Device
                };
            }
            _ => unreachable!(),
        }
    }

    fn load_device_from_selected_host(&mut self) {
        self.devices = self.cache.0[self.selected_host_index].devices.clone();

        self.device_panel_width = self
            .devices
            .iter()
            .map(|device| device.name.len())
            .max()
            .map(|width| width as u16 + Self::LIST_SELECTION_SYMBOL_LEN)
            .unwrap()
            .max(const { Self::DEVICE_LIST_TITLE.len() as u16 });

        const BORDER_SIDES: u16 = 2;
        self.popup_width = u16::max(
            Self::LIST_SPACING
                + self.host_panel_width
                + Self::LIST_SPACING
                + Self::INTER_LIST_GAP
                + Self::LIST_SPACING
                + self.device_panel_width
                + Self::LIST_SPACING,
            const { Self::KEYBINDINGS_LABEL.len() as u16 } + 2,
        ) + BORDER_SIDES;
    }

    fn selected_device(&self) -> Device {
        self.devices[self.device_list_state.selected().unwrap()].clone()
    }
}

impl Popup {
    const NO_HOST_LABEL: &str = "There is no availble host on this device";
    const POPUP_TITLE: &str = " Device selection";

    fn render_simple_message(&self, area: Rect, buf: &mut Buffer, message: &str) {
        let message_len = message.len() as u16;

        let area = responsive_centered_rect(
            area,
            Constraint::Percentage(60),
            Constraint::Min(message_len + 4),
            Constraint::Percentage(60),
            Constraint::Percentage(60),
        );
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_top(Self::POPUP_TITLE);

        let area = {
            let inner = block.inner(area);
            Clear.render(area, buf);
            block.render(area, buf);
            inner
        };

        let message_area = centered_line(area);

        message.to_line().centered().render(message_area, buf);
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        match self {
            Popup::NoHost => self.render_simple_message(area, buf, Self::NO_HOST_LABEL),
            Popup::SelectedHost(SelectedHostState {
                hosts_name,
                selected_host_index,
                devices,
                device_list_state,
                cache: _,
                host_panel_width,
                device_panel_width,
                popup_width,
                selected_panel,
            }) => {
                let area = responsive_centered_rect(
                    area,
                    Constraint::Percentage(50),
                    Constraint::Min(*popup_width),
                    Constraint::Max(*popup_width * 2),
                    Constraint::Percentage(60),
                );

                let block = Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top(Self::POPUP_TITLE)
                    .title_bottom(Line::from(SelectedHostState::KEYBINDINGS_LABEL).right_aligned());

                let area = {
                    let inner = block.inner(area);
                    Clear.render(area, buf);
                    block.render(area, buf);
                    inner
                };

                let [host_panel_area, _, device_panel_area] = Layout::horizontal([
                    Constraint::Min(*host_panel_width),
                    Constraint::Length(SelectedHostState::INTER_LIST_GAP),
                    Constraint::Min(*device_panel_width),
                ])
                .spacing(SelectedHostState::LIST_SPACING)
                .areas(area);

                let (host_list_highlight_style, device_list_highlight_style) = match selected_panel
                {
                    Panel::Host => (
                        SelectedHostState::SELECTED_HIGHLIGHT_STYLE,
                        SelectedHostState::UNSELECTED_HIGHLIGHT_STYLE,
                    ),
                    Panel::Device => (
                        SelectedHostState::UNSELECTED_HIGHLIGHT_STYLE,
                        SelectedHostState::SELECTED_HIGHLIGHT_STYLE,
                    ),
                };

                let [host_title_area, host_item_list_area] = Layout::vertical([
                    Constraint::Length(SelectedHostState::LIST_TITLE_HEIGHT),
                    Constraint::Fill(1),
                ])
                .areas(host_panel_area);

                SelectedHostState::HOST_LIST_TITLE
                    .underlined()
                    .into_centered_line()
                    .render(host_title_area, buf);

                StatefulWidget::render(
                    List::new(hosts_name.clone())
                        .highlight_spacing(HighlightSpacing::Always)
                        .highlight_symbol(SelectedHostState::LIST_SELECTION_SYMBOL)
                        .highlight_style(host_list_highlight_style),
                    host_item_list_area,
                    buf,
                    &mut ListState::default().with_selected(Some(*selected_host_index)),
                );

                let [device_title_area, device_item_list_area] = Layout::vertical([
                    Constraint::Length(SelectedHostState::LIST_TITLE_HEIGHT),
                    Constraint::Fill(1),
                ])
                .areas(device_panel_area);

                SelectedHostState::DEVICE_LIST_TITLE
                    .underlined()
                    .into_centered_line()
                    .render(device_title_area, buf);

                let devices_name = devices.iter().map(|device| device.name.clone());

                StatefulWidget::render(
                    List::new(devices_name)
                        .highlight_spacing(HighlightSpacing::Always)
                        .highlight_symbol(SelectedHostState::LIST_SELECTION_SYMBOL)
                        .highlight_style(device_list_highlight_style),
                    device_item_list_area,
                    buf,
                    device_list_state,
                );
            }
        }
    }

    pub fn map_event(&self, event: &Event) -> Option<PopupEvent> {
        match event {
            crate::event::Event::Action(action) => match action {
                Action::Cancel => Some(PopupEvent::ClosePopup),
                Action::Move(direction) => Some(PopupEvent::Move(*direction)),
                Action::Confirm => Some(PopupEvent::Select),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn handle_event(&mut self, event: PopupEvent, event_tx: Sender<Event>) {
        match event {
            PopupEvent::ClosePopup => event_tx.send(Event::ClosePopup).unwrap(),
            PopupEvent::Move(direction) => {
                if let Popup::SelectedHost(selected_host_state) = self {
                    selected_host_state.move_cursor(direction);
                }
            }
            PopupEvent::Select => {
                if let Popup::SelectedHost(state) = self {
                    if matches!(state.selected_panel, Panel::Device) {
                        event_tx
                            .send(Event::Composite(vec![
                                Event::ClosePopup,
                                Event::SetPlayingDevice(state.selected_device()),
                            ]))
                            .unwrap();
                    }
                }
            }
        }
    }
}
