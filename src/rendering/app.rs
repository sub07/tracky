use tiny_skia::Pixmap;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::{Scalar, Vec2};
use crate::rendering::font::Font;
use crate::rendering::renderer::{Renderer, SkiaRenderer};

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

const DEFAULT_SIZE_IN_CHAR: Vec2 = Vec2::new(100, 30);

pub enum AppEvent<'a, 'b, R: Renderer> {
    Init(&'a R),
    DrawRequest(&'a mut R),
    Event(WindowEvent<'b>, &'a R),
}

pub fn launch<F: FnMut(AppEvent<SkiaRenderer>) + 'static>(mut handle_event: F) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT))
        .with_min_inner_size(PhysicalSize::new(50, 50))
        .build(&event_loop)
        .unwrap();

    let font = Font::from_bytes(include_bytes!("../../font.ttf") as &[u8], 22.0);

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let PhysicalSize { width, height } = window.inner_size();

    let mut renderer = SkiaRenderer::new(
        window,
        surface,
        Pixmap::new(width, height).unwrap(),
        font,
    );

    renderer.set_window_size(renderer.glyph_size() * DEFAULT_SIZE_IN_CHAR);

    handle_event(AppEvent::Init(&renderer));

    event_loop.run(move |e, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        renderer.window.request_redraw();

        match e {
            Event::RedrawRequested(_) => {
                renderer.clear((20, 20, 20));
                // handle_event(AppEvent::DrawRequest(&mut renderer));
                renderer.present(); // Slows down window
            }
            Event::WindowEvent { event: window_event, .. } => {
                match window_event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        if let Some(pixmap) = Pixmap::new(size.width, size.height) {
                            renderer.screen = pixmap;
                            renderer.window_surface.resize(size.width, size.height).unwrap();
                        }
                        handle_event(AppEvent::Event(WindowEvent::Resized(size), &renderer));
                    }
                    e => handle_event(AppEvent::Event(e, &renderer)),
                }
            }
            _ => {}
        }
    });
}
