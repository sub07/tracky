use std::num::NonZeroU32;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::{env, io, panic, thread};

use ::log::{debug, error, info, warn};
use audio::{device, signal, Hosts};
use event::{Action, AsyncAction, Event, Text};
use model::pattern::{HexDigit, NoteName};
use ratatui::style::Color;
use ratatui::Terminal;
use ratatui_wgpu::shaders::DefaultPostProcessor;
use ratatui_wgpu::WgpuBackend;
use tracky::Tracky;
use view::popup::input::InputId;
use view::popup::{self, Popup};
use view::render_root;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::keyboard::{Key, PhysicalKey};
use winit::platform::pump_events::EventLoopExtPumpEvents;
use winit::window::{Fullscreen, Window, WindowAttributes};

mod audio;
mod event;
mod keybindings;
mod model;
mod service;
mod tracky;
mod utils;
mod view;

pub type EventSender = EventLoopProxy<Event>;

struct App<'d> {
    window: Option<Arc<Window>>,
    backend: Option<Terminal<WgpuBackend<'d, 'static>>>,
    tracky: Tracky,
    event_tx: EventSender,
}

const CHAR_PIXEL_HEIGHT: f32 = 16.0;
const CHAR_PIXEL_WIDTH: f32 = CHAR_PIXEL_HEIGHT / 2.0;

impl ApplicationHandler<Event> for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        ));

        let size = self.window.as_ref().unwrap().inner_size();
        self.backend = Some(
            Terminal::new(
                futures_lite::future::block_on(
                    ratatui_wgpu::Builder::from_font(
                        ratatui_wgpu::Font::new(include_bytes!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/fonts/CascadiaMono.ttf"
                        )))
                        .unwrap(),
                    )
                    .with_font_size_px(CHAR_PIXEL_HEIGHT as u32)
                    .with_bg_color(Color::Rgb(30, 30, 46))
                    .with_fg_color(Color::Rgb(205, 214, 244))
                    .with_width_and_height(ratatui_wgpu::Dimensions {
                        width: NonZeroU32::new(size.width).unwrap(),
                        height: NonZeroU32::new(size.height).unwrap(),
                    })
                    .build_with_target(self.window.as_ref().unwrap().clone()),
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
        if let WindowEvent::CloseRequested = event {
            self.event_tx.send_event(Event::ExitApp).unwrap();
            return;
        }

        let Some(terminal) = self.backend.as_mut() else {
            return;
        };

        match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } if event.state == ElementState::Pressed => {
                self.event_tx.send_event(Event::KeyPressed(event)).unwrap();
            }
            WindowEvent::Resized(new_size) => {
                terminal
                    .backend_mut()
                    .resize(new_size.width, new_size.height);
                // Ugly fix to avoid text rendering artifacts
                let Ok(ratatui::prelude::Size { width, height }) = terminal.size() else {
                    return;
                };
                let ideal_width = width as f32 * CHAR_PIXEL_WIDTH;
                let ideal_height = height as f32 * CHAR_PIXEL_HEIGHT;
                terminal
                    .backend_mut()
                    .resize(ideal_width as u32, ideal_height as u32);
            }
            WindowEvent::RedrawRequested => {
                terminal
                    .draw(|f| {
                        render_root(&mut self.tracky, f);
                    })
                    .unwrap();
            }
            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, mut event: Event) {
        self.window.as_ref().unwrap().request_redraw();

        macro_rules! send {
            ($e:expr) => {
                self.event_tx.send_event($e).unwrap()
            };
        }
        if let Some(popup) = self.tracky.popup_state.last_mut() {
            if let Some(unprocessed_event) = popup.handle_event(event, self.event_tx.clone()) {
                event = unprocessed_event;
            } else {
                return;
            }
        }

        match event {
            Event::KeyPressed(key_event) => {
                if let PhysicalKey::Code(key_code) = key_event.physical_key {
                    if let Some(event) = self
                        .tracky
                        .keybindings
                        .action(key_code, self.tracky.input_context())
                    {
                        send!(event);
                        return;
                    }
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
                            send!(Event::State(model::Event::SetHexField(hex_digit)));
                        }
                    }
                    keybindings::InputContext::Text => {
                        if let Some(text) = key_event.text {
                            send!(Event::Text(Text::WriteDataAtCursor(
                                text.chars().next().unwrap()
                            )));
                        }
                    }
                    _ => {}
                }
            }
            Event::Action(action) => match action {
                Action::TogglePlay => {
                    if self.tracky.state.is_playing() {
                        send!(Event::State(model::Event::StopSongPlayback));
                    } else if let Some(audio_state) = self.tracky.audio_state.as_ref() {
                        send!(Event::State(model::Event::StartSongPlayback {
                            frame_rate: audio_state.player.frame_rate,
                        }));
                    } else {
                        warn!("Select a device with F1 to play the song")
                    }
                }
                Action::Cancel => {}
                Action::Confirm => {}
                Action::RequestOpenDeviceSelectionPopup => {
                    // send!(Event::StartLoading);
                    // let event_tx_clone = self.event_tx.clone();
                    // thread::spawn(move || {
                    //     event_tx_clone
                    //         .send_event(Event::LoadingDone(AsyncAction::OpenDeviceSelectionPopup(
                    //             Hosts::load(),
                    //         )))
                    //         .unwrap();
                    // });
                    self.tracky
                        .popup_state
                        .push(Popup::Input(popup::input::Popup::new(
                            InputId::new(),
                            "My label".into(),
                            None,
                            |_| true,
                            |_| true,
                        )));
                }
                Action::Move(direction) => send!(Event::State(model::Event::MoveCursor(direction))),
                Action::Forward => todo!(),
                Action::Backward => todo!(),
                Action::ToggleFullscreen => {
                    let window = self.window.as_ref().unwrap();
                    if window.fullscreen().is_some() {
                        window.set_fullscreen(None);
                    } else {
                        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                    }
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
                event::AsyncAction::OpenDeviceSelectionPopup(hosts) => {
                    self.tracky
                        .popup_state
                        .push(Popup::AudioDeviceSelection(hosts.into()));
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
                self.tracky.state.handle_event(event.clone());
                self.tracky.send_player_state_event(event);
            }
            Event::AudioCallback(event) => self.tracky.state.handle_event(event),
            Event::ExitApp => event_loop.exit(),
            Event::StartAudioPlayer => self.tracky.start_audio_player(self.event_tx.clone()),
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
                    .handle_event(model::Event::StopSongPlayback);
            }
            Event::Text(_) => unreachable!(), // For now
            Event::TextSubmitted(id, value) => {
                info!("Text submitted: {value} with id: {id}");
            } // For now
        }
    }
}

fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    env::set_var("RUST_BACKTRACE", "1");

    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Trace)
        .filter_module("wgpu", log::LevelFilter::Off)
        .filter_module("naga", log::LevelFilter::Off)
        .filter_module("ratatui_wgpu::utils::text_atlas", log::LevelFilter::Off)
        .init();

    let mut tracky = Tracky::new();

    tracky.state.handle_event(model::Event::SetNoteField {
        note: NoteName::A,
        octave_modifier: 0,
    });

    let event_loop = EventLoop::<Event>::with_user_event().build()?;
    let event_tx = event_loop.create_proxy();

    event_tx
        .send_event(Event::SetPlayingDevice(device::default_output().unwrap()))
        .unwrap();
    event_tx.send_event(Event::StartAudioPlayer).unwrap();

    let mut app = App {
        tracky,
        backend: None,
        window: None,
        event_tx,
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app)?;
    Ok(())
}
