#![feature(associated_type_defaults)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(variant_count)]
#![windows_subsystem = "windows"]

extern crate core;

use sdl2::keyboard::{Keycode};

use crate::app::{Event, launch};
use crate::model::{Direction};
use crate::model::pattern::Pattern;
use crate::theme::Theme;
use crate::view::Draw;

mod font_atlas;
mod renderer;
mod color;
mod app;
mod model;
mod view;
mod key_bindings;
mod theme;

fn main() {
    let mut patterns = Pattern::new(6, 15);
    let mut mouse_pos = (0, 0);
    let dark_theme = Theme::default_dark();

    launch(|event| {
        match event {
            Event::Event(sdl2::event::Event::KeyDown { keycode, .. }) => {
                match keycode {
                    Some(Keycode::Down) => {
                        patterns.move_cursor(Direction::Down)
                    }
                    Some(Keycode::Up) => {
                        patterns.move_cursor(Direction::Up)
                    }
                    Some(Keycode::Left) => {
                        patterns.move_cursor(Direction::Left)
                    }
                    Some(Keycode::Right) => {
                        patterns.move_cursor(Direction::Right)
                    }
                    _ => {}
                }
            }
            Event::Event(sdl2::event::Event::MouseMotion { x, y, .. }) => {
                mouse_pos = (x, y);
            }
            Event::DrawRequest(renderer) => {
                patterns.draw(renderer, 0, 0, &dark_theme, ());
            }
            _ => {}
        }
    });
}
