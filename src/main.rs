#![feature(associated_type_defaults)]
#![windows_subsystem = "windows"]

use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Scancode;
use sdl2::surface::Surface;

use crate::draw::Draw;
use crate::error::Error;
use crate::pattern::{Note, Pattern};
use crate::renderer::Renderer;

mod error;
mod pattern;
mod font_atlas;
mod draw;
mod renderer;
mod color;

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init().map_err(|e| Error::SdlError(e))?;
    let mut window = sdl.video().unwrap()
        .window("Tracky", 1200, 800)
        .position_centered()
        .resizable()
        .maximized()
        .build()
        .unwrap();

    window.set_icon(Surface::from_file("icon.png").unwrap());

    let renderer = window
        .into_canvas()
        .software()
        .build()
        .unwrap();

    let mut events = sdl.event_pump().unwrap();
    let texture_creator = renderer.texture_creator();

    let mut renderer = Renderer::new(
        renderer,
        &texture_creator,
        "font.ttf",
        24,
        "0123456789-ABCDEFGHIJKLMNOPQRSTUVWXYZ#",
    );

    let mut pattern = Pattern::new(12);
    let mut cursor = 0;

    'gameLoop: loop {
        if let Some(event) = events.wait_event_timeout(100) {
            match event {
                Event::Quit { .. } => {
                    break 'gameLoop;
                }
                Event::KeyDown { scancode, .. } => {
                    match scancode {
                        Some(Scancode::Down) => {
                            cursor = if cursor == pattern.len() - 1 { 0 } else { cursor + 1 };
                        }
                        Some(Scancode::Up) => {
                            cursor = if cursor == 0 { pattern.len() - 1 } else { cursor - 1 };
                        }
                        _ => {}
                    }
                    let note = match scancode {
                        Some(Scancode::Q) => Some(Note::C),
                        Some(Scancode::Num2) => Some(Note::CSharp),
                        Some(Scancode::W) => Some(Note::D),
                        Some(Scancode::Num3) => Some(Note::DSharp),
                        Some(Scancode::E) => Some(Note::E),
                        Some(Scancode::R) => Some(Note::F),
                        Some(Scancode::Num5) => Some(Note::FSharp),
                        Some(Scancode::T) => Some(Note::G),
                        Some(Scancode::Num6) => Some(Note::GSharp),
                        Some(Scancode::Y) => Some(Note::A),
                        Some(Scancode::Num7) => Some(Note::ASharp),
                        Some(Scancode::U) => Some(Note::B),
                        _ => None
                    };
                    if let Some(note) = note {
                        pattern.set(cursor, note)?;
                    }
                }
                _ => {}
            }
        }

        renderer.clear((20, 20, 20));
        pattern.draw(&mut renderer, 100, 300, cursor);
        renderer.present();
    }

    Ok(())
}
