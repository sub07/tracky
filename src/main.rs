#![feature(associated_type_defaults)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![windows_subsystem = "windows"]

extern crate core;

use sdl2::keyboard::Scancode;

use crate::app::{Event, launch};
use crate::model::{Direction, Note};
use crate::model::patterns::Patterns;
use crate::view::Draw;

mod font_atlas;
mod renderer;
mod color;
mod app;
mod model;
mod view;

fn main() {
    let mut patterns = Patterns::new(6, 15);
    let mut mouse_pos = (0, 0);

    launch(|event| {
        match event {
            Event::Event(sdl2::event::Event::KeyDown { scancode, .. }) => {
                match scancode {
                    Some(Scancode::Down) => {
                        patterns.move_cursor(Direction::Down)
                    }
                    Some(Scancode::Up) => {
                        patterns.move_cursor(Direction::Up)
                    }
                    Some(Scancode::Left) => {
                        patterns.move_cursor(Direction::Left)
                    }
                    Some(Scancode::Right) => {
                        patterns.move_cursor(Direction::Right)
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
            }
            Event::Event(sdl2::event::Event::MouseMotion { x, y, .. }) => {
                mouse_pos = (x, y);
            }
            Event::DrawRequest(renderer) => {
                patterns.draw(renderer, 50, 50, ());
            }
            _ => {}
        }
    });
}
