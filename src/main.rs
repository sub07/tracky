#![feature(associated_type_defaults)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(variant_count)]
#![windows_subsystem = "windows"]

extern crate core;


use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use crate::app::{Event, launch};
use crate::controller::patterns::PatternsController;
use crate::model::patterns::Patterns;
use crate::theme::Theme;
use crate::view::Draw;
use crate::view::patterns::PatternsView;

mod mono_font_atlas;
mod renderer;
mod app;
mod model;
mod view;
mod key_bindings;
mod theme;
mod controller;
mod game_loop_metrics;

fn main() {
    let patterns = Rc::new(RefCell::new(Patterns::new(64, 64)));
    let pattern_view = PatternsView::new(patterns.clone());
    let controller = PatternsController::default();
    let dark_theme = Theme::default_dark();

    launch(|event| {
        match event {
            Event::Event(event, _) => {
                let mut model = patterns.borrow_mut();
                controller.handle_event(model.deref_mut(), event);
            }
            Event::DrawRequest(renderer, _) => {
                pattern_view.draw(renderer, 0, 0, &dark_theme, ());
            }
        }
    });
}
