use std::num::NonZeroU32;
use std::sync::Arc;
use std::{env, panic, thread};

use ::log::{error, info, warn};
use audio::device::{self, Devices};
use event::{Action, AsyncAction, Event, EventAware, Text};
use model::pattern::{HexDigit, NoteName};
use ratatui::Terminal;
use ratatui_wgpu::WgpuBackend;
use tracky::Tracky;
use view::popup::Popup;
use view::render_root;
use view::screen::Screen;
use view::theme::THEME;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::keyboard::{Key, PhysicalKey};
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

impl ApplicationHandler<Event> for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Tracky")
                        .with_inner_size(PhysicalSize::new(1600, 900)),
                )
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
                    .with_font_size_px(18)
                    .with_bg_color(THEME.background)
                    .with_fg_color(THEME.on_background)
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
        if let Some(popup) = &mut self.tracky.current_popup {
            if let Some(unprocessed_event) = popup.handle_event(event, self.event_tx.clone()) {
                event = unprocessed_event;
            } else {
                return;
            }
        }

        match &mut self.tracky.current_screen {
            Screen::DeviceSelection(state) => {
                if let Some(unprocessed_event) = state.handle_event(event, self.event_tx.clone()) {
                    event = unprocessed_event;
                } else {
                    return;
                }
            }
            Screen::SongEditor => {}
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
                    send!(Event::StartLoading);
                    let event_tx_clone = self.event_tx.clone();
                    thread::spawn(move || {
                        event_tx_clone
                            .send_event(Event::LoadingDone(AsyncAction::OpenDeviceSelectionPopup(
                                Devices::load(),
                            )))
                            .unwrap();
                    });
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
                event::AsyncAction::OpenDeviceSelectionPopup(devices) => {
                    self.tracky.current_screen = Screen::DeviceSelection(devices.into())
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

    // event_tx
    //     .send_event(Event::SetPlayingDevice(device::default_output().unwrap()))
    //     .unwrap();
    // event_tx.send_event(Event::StartAudioPlayer).unwrap();

    // println!("{:#?}", device::Devices::load());

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
