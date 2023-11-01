use iter_tools::Itertools;

use crate::{
    audio::signal::StereoSignal,
    model::{
        audio_channel::AudioChannel,
        pattern::{PatternView, Patterns},
    },
};

pub fn handle_patterns(patterns: &Patterns, sample_rate: f32, bps: f32) -> StereoSignal {
    let nb_line_song = patterns.lengths.iter().sum::<u32>();
    let song_duration = AudioChannel::compute_duration(bps, nb_line_song).1;
    let mut master_signal = StereoSignal::new(song_duration, sample_rate);

    let audio_channels =
        (0..patterns.nb_channel).map(|_| AudioChannel::new(bps, sample_rate, nb_line_song));

    patterns
        .tracks()
        .zip(audio_channels)
        .map(|(track, mut audio_channel)| {
            for column in track {
                audio_channel.handle_lines(column.lines);
            }
            audio_channel
        })
        .for_each(|audio_channel| master_signal += audio_channel.signal());

    master_signal
}
