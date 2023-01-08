use sdl2::image::LoadSurface;
use sdl2::surface::Surface;

use crate::renderer::Renderer;

pub enum Event<'a, 'b> {
    DrawRequest(&'a mut Renderer<'b>),
    Event(sdl2::event::Event),
}

pub fn launch<F: FnMut(Event)>(mut handle_event: F) {
    let sdl = sdl2::init().unwrap();
    let mut window = sdl.video().unwrap()
        .window("Tracky", 1000, 800)
        .position_centered()
        .resizable()
        // .maximized()
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

    'gameLoop: loop {
        // let mut events_vec = Vec::new();
        // events_vec.push(events.wait_event());
        // events_vec.extend(events.poll_iter().collect::<Vec<_>>());
        for event in events.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event { break 'gameLoop; } else { handle_event(Event::Event(event)) }
        }
        renderer.clear((20, 20, 20));
        handle_event(Event::DrawRequest(&mut renderer));
        renderer.present();
    }
}
