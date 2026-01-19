#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release builds
#![warn(clippy::pedantic, clippy::nursery, clippy::dbg_macro)]
#![allow(
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::default_trait_access,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use std::num::NonZeroU32;
use std::sync::Arc;
use std::{env, panic, thread};

use app::Tracky;
use audio::device::{self, Devices};
use event::{Action, AsyncAction, Event, Text};
use log::{error, info, warn};
use model::pattern::{HexDigit, NoteName};
use ratatui::Terminal;
use ratatui_wgpu::WgpuBackend;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::keyboard::{Key, ModifiersState, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowAttributes};

use crate::app::view::render_root;
use crate::app::view::screen::{Screen, device_selection};
use crate::app::view::theme::THEME;
use crate::graphic_post_processor::BackgroundColorEdgesPostProcessor;

mod app;
mod audio;
mod event;
mod graphic_post_processor;
mod keybindings;
mod model;
mod service;
mod stats;
mod utils;

pub type EventSender = EventLoopProxy<Event>;

struct App<'d> {
    window: Option<Arc<Window>>,
    backend: Option<Terminal<WgpuBackend<'d, 'static, BackgroundColorEdgesPostProcessor>>>,
    tracky: Tracky,
    event_sender: EventSender,
    modifiers_state: ModifiersState,
}

impl ApplicationHandler<Event> for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Tracky")
                        .with_inner_size(PhysicalSize::new(1600, 900)),
                )
                .unwrap(),
        );
        self.window = Some(window.clone());
        let window_size = window.inner_size();
        let font_size = (18.0 * window.scale_factor()).round() as u32;
        let bg_color = THEME.normal.bg.unwrap();
        self.backend = Some(
            Terminal::new(
                futures_lite::future::block_on(
                    ratatui_wgpu::Builder::from_font_and_user_data(
                        ratatui_wgpu::Font::new(include_bytes!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/fonts/CascadiaMono.ttf"
                        )))
                        .unwrap(),
                        bg_color,
                    )
                    .with_font_size_px(font_size)
                    .with_bg_color(bg_color)
                    .with_fg_color(THEME.normal.fg.unwrap())
                    .with_width_and_height(ratatui_wgpu::Dimensions {
                        width: NonZeroU32::new(window_size.width).unwrap(),
                        height: NonZeroU32::new(window_size.height).unwrap(),
                    })
                    .build_with_target(window),
                )
                .unwrap(),
            )
            .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if event == WindowEvent::CloseRequested {
            self.event_sender.send_event(Event::ExitApp).unwrap();
            return;
        }

        let Some(terminal) = self.backend.as_mut() else {
            return;
        };

        match event {
            WindowEvent::ModifiersChanged(modifers) => self.modifiers_state = modifers.state(),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } if event.state == ElementState::Pressed => {
                self.event_sender
                    .send_event(Event::KeyPressed(self.modifiers_state, event))
                    .unwrap();
            }
            WindowEvent::Resized(new_size) => {
                terminal
                    .backend_mut()
                    .resize(new_size.width, new_size.height);
            }
            // WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer }
            WindowEvent::RedrawRequested => {
                self.tracky.stats.record_render();
                terminal
                    .draw(|f| {
                        render_root(&mut self.tracky, f);
                    })
                    .unwrap();
            }
            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Event) {
        self.tracky.stats.record_event(&event);
        self.window.as_ref().unwrap().request_redraw();

        macro_rules! send {
            ($e:expr) => {
                self.event_sender.send_event($e).unwrap()
            };
        }

        match event {
            Event::KeyPressed(modifiers_state, key_event) => {
                if let PhysicalKey::Code(key_code) = key_event.physical_key
                    && let Some(action) = self.tracky.keybindings.action(
                        modifiers_state,
                        key_code,
                        self.tracky.input_context(),
                    )
                {
                    send!(Event::Action(action));
                    return;
                }

                match self.tracky.input_context() {
                    keybindings::InputContext::Hex => {
                        if let KeyEvent {
                            logical_key: Key::Character(c),
                            ..
                        } = key_event
                        {
                            let hex_digit = match c.as_str().to_lowercase().as_str() {
                                "a" => HexDigit::HEX_A,
                                "b" => HexDigit::HEX_B,
                                "c" => HexDigit::HEX_C,
                                "d" => HexDigit::HEX_D,
                                "e" => HexDigit::HEX_E,
                                "f" => HexDigit::HEX_F,
                                _ => return,
                            };
                            send!(Event::State(model::Command::SetHexField(hex_digit)));
                        }
                    }
                    keybindings::InputContext::Text => {
                        if let Some(text) = key_event.text {
                            send!(Event::Action(Action::Text(Text::WriteDataAtCursor(text))));
                        }
                    }
                    _ => {}
                }
            }
            Event::Action(action) => match action {
                Action::RequestChangeScreenToDeviceSelection => {
                    send!(Event::StartLoading);
                    let event_tx_clone = self.event_sender.clone();
                    thread::spawn(move || {
                        event_tx_clone
                            .send_event(Event::LoadingDone(
                                AsyncAction::GetDevices(Devices::load()),
                            ))
                            .unwrap();
                    });
                }
                Action::RequestChangeScreenToSongEditor => {
                    send!(Event::ChangeScreen(Screen::SongEditor));
                }
                Action::ToggleFullscreen => {
                    let window = self.window.as_ref().unwrap();
                    if window.fullscreen().is_some() {
                        window.set_fullscreen(None);
                    } else {
                        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                    }
                }
                action => {
                    self.tracky.handle_action(action);
                }
            },
            Event::Panic(error) => {
                panic!("{error:?}");
            }
            Event::Composite(events) => {
                for event in events {
                    send!(event);
                }
            }
            Event::Resize { width, height } => info!("{width}x{height}"),
            Event::AsyncAction(async_action) => match async_action {
                event::AsyncAction::GetDevices(devices) => {
                    send!(Event::ChangeScreen(Screen::DeviceSelection(
                        device_selection::State::from(devices)
                    )));
                }
            },
            Event::StartLoading => self.tracky.loader_count += 1,
            Event::LoadingDone(async_action) => {
                self.tracky.loader_count = self.tracky.loader_count.saturating_sub(1);
                send!(Event::AsyncAction(async_action));
            }
            Event::ClosePopup => self.tracky.close_popup(),
            Event::SetPlayingDevice(device) => self.tracky.selected_output_device = Some(device),
            Event::State(event) => {
                self.tracky.state.handle_command(event.clone());
                self.tracky.send_player_state_event(event);
            }
            Event::AudioCallback(event) => self.tracky.state.handle_command(event),
            Event::ExitApp => {
                self.tracky.stats.print_stats();
                self.tracky.teardown();
                event_loop.exit();
            }
            Event::StartAudioPlayer => self.tracky.start_audio_player(self.event_sender.clone()),
            Event::RequestRedraw => {}
            Event::StopAudioPlayer(error) => {
                if let Some(err) = error {
                    error!("Audio player stopped: {err}");
                } else {
                    info!("Audio played stopped");
                }
                self.tracky.stop_audio_player();
                self.tracky
                    .state
                    .handle_command(model::Command::StopSongPlayback);
            }
            Event::Text(_) => unreachable!(),

            Event::ChangeScreen(screen) => {
                self.tracky.change_screen(screen);
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Trace)
        .filter_module("wgpu", log::LevelFilter::Off)
        .filter_module("naga", log::LevelFilter::Off)
        .filter_module("ratatui_wgpu::utils::text_atlas", log::LevelFilter::Off)
        .init();

    let event_loop = EventLoop::<Event>::with_user_event().build()?;
    let event_sender = event_loop.create_proxy();

    let mut tracky = Tracky::new(event_sender.clone());

    tracky.state.handle_command(model::Command::SetNoteField {
        note: NoteName::A,
        octave_modifier: 0,
    });
    tracky.state.handle_command(model::Command::ClearChannels);

    if let Some(default_device) = device::default_output() {
        event_sender
            .send_event(Event::SetPlayingDevice(default_device))
            .unwrap();
        event_sender.send_event(Event::StartAudioPlayer).unwrap();
    } else {
        error!("Default device could not be found");
    }

    let mut app = App {
        tracky,
        backend: None,
        window: None,
        event_sender,
        modifiers_state: ModifiersState::empty(),
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app)?;
    Ok(())
}
