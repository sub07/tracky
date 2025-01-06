use std::time::Duration;

use log::{error, info};

use crate::{
    audio::{mixer::Mixer, player::AudioPlayer, signal::StereoSignal},
    model::channel::Channel,
    tracky::Tracky,
};

pub struct Playback {
    pub player: AudioPlayer,
    channels: Vec<Channel>,
    master: Mixer,
    line_audio_buffer: StereoSignal,
    current_line: usize,
    line_duration: Duration,
    time_since_last_line: Duration,
    sink: StereoSignal,
}

impl Tracky {
    fn make_player(&self) -> anyhow::Result<AudioPlayer> {
        self.selected_output_device
            .clone()
            .ok_or_else(|| anyhow!("No output device selected"))
            .and_then(Player::with_device);
        AudioPlayer::with_default_device()
    }

    fn setup_realtime_playback(&mut self) -> anyhow::Result<()> {
        let player = self.make_player()?;
        let line_duration = Duration::from_secs_f32(1.0 / self.line_per_second);
        let line_audio_buffer = StereoSignal::new(line_duration, player.frame_rate);
        let master = Mixer::new(player.frame_rate);
        let channels = vec![Channel::new(); self.patterns.channel_count as usize];
        let sink = StereoSignal::new(Duration::ZERO, player.frame_rate);
        // player.play()?;
        self.playback_state = Some(Playback {
            player,
            channels,
            master,
            line_audio_buffer,
            current_line: 0,
            time_since_last_line: line_duration * 2,
            line_duration,
            sink,
        });

        Ok(())
    }

    pub fn handle_toggle_playback(&mut self) {
        if let Err(err) = self.toggle_playback() {
            error!("{err}");
        }
    }

    pub fn toggle_playback(&mut self) -> anyhow::Result<()> {
        if self.is_playing() {
            self.stop_playback()?;
        } else {
            self.start_playback()?;
        }
        Ok(())
    }

    pub fn start_playback(&mut self) -> anyhow::Result<()> {
        self.setup_realtime_playback()?;
        Ok(())
    }

    pub fn stop_playback(&mut self) -> anyhow::Result<()> {
        if let Some(mut playback) = self.playback_state.take() {
            info!("Stopping playback");
            // playback.player.stop()?;
        }

        Ok(())
    }

    pub fn is_playing(&self) -> bool {
        self.playback_state.is_some()
    }

    pub fn playback_tick(&mut self, delta: Duration) -> anyhow::Result<()> {
        if let Some(ref mut playback) = self.playback_state {
            playback.time_since_last_line += delta;
            while playback.time_since_last_line >= playback.line_duration {
                playback.time_since_last_line -= playback.line_duration;
                playback.master.reset();
// 
                for (line, channel) in self
                    .patterns
                    .current_pattern_row(playback.current_line)?
                    .zip(&mut playback.channels)
                {
                    channel.setup_line(line);
                    channel.collect_signal(&mut playback.line_audio_buffer);
                    playback.master.mix(&playback.line_audio_buffer);
                }
// 
                debug_assert_eq!(
                    playback.line_audio_buffer.duration(),
                    playback.master.output.duration(),
                );
// 
                playback.sink.append_signal(&playback.master.output)?;
// 
                playback.player.queue_signal(&playback.master.output);
// 
                playback.current_line += 1;
                if playback.current_line as i32 == self.patterns.channel_len {
                    break;
                }
            }
            if playback.current_line as i32 == self.patterns.channel_len {
                self.stop_playback()?;
            }
        }
        Ok(())
    }
}
