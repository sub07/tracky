use iter_tools::Itertools;

use crate::{model::{pattern::PatternView, audio_channel::AudioChannel}, audio::model::signal::StereoSignal};

pub fn handle_pattern(pattern: &PatternView, bps: f32, sample_rate: f32) -> StereoSignal {
    let mut audio_channels = (0..pattern.nb_column).map(|_| AudioChannel::new(bps, sample_rate, pattern.len)).collect_vec();
    let mut pattern_signal = StereoSignal::new(AudioChannel::compute_duration(bps, pattern.len).1, sample_rate);

    for (column, audio_channel) in pattern.columns().zip(&mut audio_channels) {
        audio_channel.handle_lines(column.lines);
        pattern_signal += audio_channel.signal();
    }

    pattern_signal
}