use std::num::NonZeroU32;
use std::sync::Arc;
use std::{env, panic, thread};

use ::log::{error, info, warn};
use audio::device::{self, Devices};
use event::{Action, AsyncAction, Event, HandleAction, Text};
use model::pattern::{HexDigit, NoteName};
use ratatui::Terminal;
use ratatui_wgpu::WgpuBackend;
use tracky::Tracky;
use view::popup::{change_volume, Popup};
use view::render_root;
use view::screen::{device_selection, Screen};
use view::theme::THEME;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::keyboard::{Key, ModifiersState, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowAttributes};

use crate::event::GlobalAction;
use crate::keybindings::{InputContext, InputLocation, InputType};
use crate::utils::BackgroundColorEdgesPostProcessor;
use crate::view::screen::{self, song_editor, ScreenAction};

mod audio;
mod event;
mod keybindings;
mod model;
mod service;
mod stats;
mod tracky;
mod utils;
mod view;

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
        let font_size = 18.0 * window.scale_factor();
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
                    .with_font_size_px(font_size as u32)
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
        if let WindowEvent::CloseRequested = event {
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
                let input_type = self.tracky.input_location();
                if let PhysicalKey::Code(key_code) = key_event.physical_key {
                    let input_location = match self.tracky.current_screen {
                        Screen::DeviceSelection(_) => InputLocation::Global,
                        Screen::SongEditor(_) => InputLocation::SongEditor,
                    };
                    if let Some(action) = self.tracky.keybindings.action(InputContext(
                        input_location,
                        input_type,
                        modifiers_state,
                        key_code,
                    )) {
                        send!(Event::Action(action));
                        return;
                    }
                }

                if let InputType::Text = input_type {
                    if let Some(text) = key_event.text {
                        send!(Event::Text(Text::WriteDataAtCursor(
                            text.chars().next().unwrap()
                        )));
                    }
                }
            }
            Event::Action(action) => match action {
                Action::Global(ref global_action) => match global_action {
                    GlobalAction::RequestChangeScreenToDeviceSelection => {
                        send!(Event::StartLoading);
                        let event_tx_clone = self.event_sender.clone();
                        thread::spawn(move || {
                            event_tx_clone
                                .send_event(Event::LoadingDone(AsyncAction::GetDevices(
                                    Devices::load(),
                                )))
                                .unwrap();
                        });
                    }
                    GlobalAction::RequestChangeScreenToSongEditor => {
                        self.tracky.current_screen =
                            Screen::SongEditor(song_editor::State::default());
                    }
                    _ => self.tracky.current_screen.update(
                        &self.tracky,
                        action,
                        self.event_sender.clone(),
                    ),
                },
                Action::SongScreen(action) => {
                    if let Screen::SongEditor(state) = self.tracky.current_screen {
                        state.update(
                            &self.tracky,
                            ScreenAction::Screen(action),
                            self.event_sender.clone(),
                        )
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
                event::AsyncAction::GetDevices(devices) => {
                    send!(Event::ChangeScreen(Screen::DeviceSelection(
                        device_selection::State::from(devices)
                    )))
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
            Event::ToggleFullscreen => {
                let window = self.window.as_ref().unwrap();
                if window.fullscreen().is_some() {
                    window.set_fullscreen(None);
                } else {
                    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                }
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

    let mut tracky = Tracky::new();

    tracky.state.handle_command(model::Command::SetNoteField {
        note: NoteName::A,
        octave_modifier: 0,
    });
    tracky.state.handle_command(model::Command::ClearChannels);

    let event_loop = EventLoop::<Event>::with_user_event().build()?;
    let event_tx = event_loop.create_proxy();

    if let Some(default_device) = device::default_output() {
        event_tx
            .send_event(Event::SetPlayingDevice(default_device))
            .unwrap();
        event_tx.send_event(Event::StartAudioPlayer).unwrap();
    } else {
        error!("Default device could not be found")
    }

    let mut app = App {
        tracky,
        backend: None,
        window: None,
        event_sender: event_tx,
        modifiers_state: ModifiersState::empty(),
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app)?;
    Ok(())
}
