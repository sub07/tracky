#![feature(associated_type_defaults)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(variant_count)]
#![feature(slice_as_chunks)]
#![windows_subsystem = "windows"]

extern crate core;

use std::time::Duration;
use rust_utils::vector::Vector;

use crate::audio::sound::Sound;
use crate::audio::stream::AudioStream;
use crate::controller::patterns::PatternsController;
use crate::model::pattern::patterns::Patterns;
use crate::rendering::app::{AppEvent, launch};
use crate::theme::Theme;
use crate::view::Draw;
use crate::view::patterns::PatternsDrawData;

mod model;
mod view;
mod key_bindings;
mod theme;
mod controller;
mod game_loop_metrics;
mod audio;
mod rendering;

type Scalar = i32;
type Vec2 = Vector<Scalar, 2>;

fn main() -> anyhow::Result<()> {
    // let mut patterns = Patterns::new(64, 64);
    // let controller = PatternsController::default();
    // let dark_theme = Theme::default_dark();

    let piano_sound = Sound::from_wav("piano.wav").unwrap();
    let mut stream = AudioStream::new().unwrap();
    stream.add_sound(&piano_sound);

    std::thread::sleep(Duration::from_secs(5));

    // launch(move |event| {
    //     match event {
    //         AppEvent::Init(_) => {}
    //         AppEvent::DrawRequest(renderer) => {
    //             patterns.draw(renderer, 0, 0, &dark_theme, PatternsDrawData::new());
    //         }
    //         AppEvent::Event(event, _) => {
    //             controller.handle_event(&mut patterns, event);
    //         }
    //     }
    // });

    Ok(())
}
