#![feature(associated_type_defaults)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(variant_count)]
#![windows_subsystem = "windows"]

extern crate core;

use rust_utils::vector::Vector;

use crate::app::{Event, launch};
use crate::controller::patterns::PatternsController;
use crate::model::pattern::patterns::Patterns;
use crate::theme::Theme;
use crate::view::Draw;
use crate::view::patterns::PatternsDrawData;

mod mono_font_atlas;
mod renderer;
mod app;
mod model;
mod view;
mod key_bindings;
mod theme;
mod controller;
mod game_loop_metrics;

type Vec2 = Vector<i32, 2>;

fn main() {
    let mut patterns = Patterns::new(64, 64);
    let controller = PatternsController::default();
    let dark_theme = Theme::default_dark();
    let pattern_x = 0;
    let pattern_y = 0;

    launch(|event| {
        match event {
            Event::Event(event, _, _) => {
                controller.handle_event(&mut patterns, event);
            }
            Event::DrawRequest(renderer, redraw) => {
                *redraw = true;
                patterns.draw(renderer, pattern_x, pattern_y, &dark_theme, PatternsDrawData::new());
            }
        }
    });
}
