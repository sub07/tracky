use std::time::Duration;

use crate::{
    audio::{mixer::Mixer, signal::StereoSignal},
    model::channel::Channel,
};

#[derive(Clone)]
pub struct SongPlayback {
    pub channels: Vec<Channel>,
    pub master: Mixer,
    pub current_line: usize,
    pub line_audio_signal: StereoSignal,
    pub line_duration: Duration,
    pub time_since_last_line: Duration,
}
//             while playback.time_since_last_line >= playback.line_duration {
//                 playback.time_since_last_line -= playback.line_duration;
//                 playback.master.reset();
//                 //
//                 for (line, channel) in self
//                     .patterns
//                     .current_pattern_row(playback.current_line)?
//                     .zip(&mut playback.channels)
//                 {
//                     channel.setup_line(line);
//                     channel.collect_signal(&mut playback.line_audio_buffer);
//                     playback.master.mix(&playback.line_audio_buffer);
//                 }
//                 //
//                 debug_assert_eq!(
//                     playback.line_audio_buffer.duration(),
//                     playback.master.output.duration(),
//                 );
//                 //
//                 playback.sink.append_signal(&playback.master.output)?;
//                 //
//                 playback.player.queue_signal(&playback.master.output);
//                 //
//                 playback.current_line += 1;
//                 if playback.current_line as i32 == self.patterns.channel_len {
//                     break;
//                 }
//             }
//             if playback.current_line as i32 == self.patterns.channel_len {
//                 self.stop_playback()?;
//             }
//         }
