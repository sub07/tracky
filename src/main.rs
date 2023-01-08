#![feature(associated_type_defaults)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(variant_count)]
#![windows_subsystem = "windows"]

extern crate core;


use crate::app::{Event, launch};
use crate::controller::patterns::PatternsController;
use crate::model::patterns::Patterns;
use crate::theme::Theme;
use crate::view::Draw;

mod mono_font_atlas;
mod renderer;
mod color;
mod app;
mod model;
mod view;
mod key_bindings;
mod theme;
mod controller;
mod game_loop_metrics;

fn main() {
    let mut patterns = Patterns::new(9, 64);
    let controller = PatternsController::default();
    let dark_theme = Theme::default_dark();

    launch(|event| {
        match event {
            Event::Event(event, _) => {
                controller.handle_event(&mut patterns, event);
            }
            Event::DrawRequest(renderer, _) => {
                patterns.draw(renderer, 0, 0, &dark_theme, ());
            }
        }
    });
}
