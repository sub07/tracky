use std::time::Duration;

use sdl2::image::LoadSurface;
use sdl2::surface::Surface;

use crate::game_loop_metrics::GameLoopMetrics;
use crate::renderer::Renderer;

pub enum Event<'a, 'b> {
    DrawRequest(&'a mut Renderer<'b>, &'a mut bool),
    Event(sdl2::event::Event, &'a mut bool),
}

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

const DEFAULT_CHAR_WIDTH: u32 = 100;
const DEFAULT_CHAR_HEIGHT: u32 = 30;

pub fn launch<F: FnMut(Event)>(mut handle_event: F) {
    let sdl = sdl2::init().unwrap();
    let mut window = sdl.video().unwrap()
        .window("Tracky", DEFAULT_WIDTH, DEFAULT_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    window.set_icon(Surface::from_file("icon.png").unwrap());

    let canvas = window
        .into_canvas()
        .software()
        .build()
        .unwrap();

    let mut events = sdl.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut renderer = Renderer::new(
        canvas,
        &texture_creator,
        "font.ttf",
        22,
        "0123456789-.ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz#/",
    );

    renderer.set_size(DEFAULT_CHAR_WIDTH * renderer.glyph_width(), DEFAULT_CHAR_HEIGHT * renderer.glyph_height());

    let mut game_loop_metrics = GameLoopMetrics::new(Duration::from_secs(1));

    let mut redraw = false;

    'gameLoop: loop {
        game_loop_metrics.update().unwrap();
        renderer.set_window_title(format!("FPS: {}", game_loop_metrics.fps()));

        let events = if redraw {
            redraw = false;
            events.poll_iter().collect::<Vec<_>>()
        } else {
            let mut events_vec = Vec::new();
            events_vec.push(events.wait_event());
            events_vec.extend(events.poll_iter().collect::<Vec<_>>());
            events_vec
        };

        for event in events {
            if let sdl2::event::Event::Quit { .. } = event { break 'gameLoop; } else { handle_event(Event::Event(event, &mut redraw)) }
        }

        renderer.clear((20, 20, 20));
        handle_event(Event::DrawRequest(&mut renderer, &mut redraw));
        renderer.present();
    }
}
