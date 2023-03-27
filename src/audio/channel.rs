use crate::audio::sound::Sound;
use crate::model::Bpm;
use crate::model::pattern::column::Column;
use crate::model::sample::Sample;

struct Channel {
    data: Sound,
    current_sample: Sample,
    current_volume: u8,
    should_play: bool,
    freq: f32,
}

fn note_to_multiplier(semitone: i32) -> f64 {
    const NOTE_MUL: f64 = 1.0594630943593;
    NOTE_MUL.powi(semitone)
}

impl Channel {
    pub fn update(&mut self, column: &Column, bpm: Bpm) {
        let bps = bpm.value() / 60.0;

    }
}
