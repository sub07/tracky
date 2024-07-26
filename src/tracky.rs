use std::time::Duration;

use anyhow::anyhow;
use itertools::Itertools;
use log::{error, info, warn};

use crate::{
    audio::{self, mixer::Mixer, player::Player, signal::StereoSignal},
    keybindings::{InputContext, KeyBindings},
    model::{channel::Channel, pattern::Patterns},
    view::popup::Popup,
    DEBUG,
};

struct Playback {
    player: Player,
    // channels: Vec<Channel>,
    // master: Mixer,
}

pub struct Tracky {
    pub running: bool,
    pub patterns: Patterns,
    pub display_log_console: bool,
    pub keybindings: KeyBindings,
    pub selected_output_device: Option<audio::device::Device>,
    pub popup_state: Option<Popup>,
    pub line_per_second: f32,
    playback_state: Option<Playback>,
    pub poll_event_timeout: Option<Duration>,
    time_acc: Duration,
}

impl Default for Tracky {
    fn default() -> Self {
        Self {
            running: true,
            patterns: Default::default(),
            display_log_console: DEBUG,
            keybindings: Default::default(),
            selected_output_device: None,
            popup_state: None,
            playback_state: None,
            line_per_second: 4.0,
            poll_event_timeout: None,
            time_acc: Duration::ZERO,
        }
    }
}

impl Tracky {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exit(&mut self) {
        self.running = false;
    }

    pub fn set_poll_mode(&mut self) {
        const EVENT_POLL_TIMEOUT: u64 = 50;
        self.poll_event_timeout = Some(Duration::from_millis(EVENT_POLL_TIMEOUT));
    }

    pub fn set_wait_mode(&mut self) {
        self.poll_event_timeout = None;
    }

    pub fn tick(&mut self, delta: Duration) {
        self.time_acc += delta;
        info!("{:?}", self.time_acc);
    }

    pub fn input_context(&self) -> crate::keybindings::InputContext {
        if self.popup_state.is_some() {
            InputContext::Popup
        } else {
            self.patterns.current_input_context()
        }
    }

    pub fn close_popup(&mut self) {
        self.popup_state = None;
    }

    fn make_player(&self) -> anyhow::Result<Player> {
        self.selected_output_device
            .clone()
            .ok_or_else(|| anyhow!("No output device selected"))
            .and_then(Player::with_device)
    }

    // TODO Move to service module
    fn setup_playback(&mut self) -> anyhow::Result<()> {
        let mut player = self.make_player()?;
        let mut master = Mixer::new(player.frame_rate);
        let mut line_audio_buffer = StereoSignal::new(
            Duration::from_secs_f32(1.0 / self.line_per_second),
            player.frame_rate,
        );

        let mut channels = self
            .patterns
            .current_pattern_channels()
            .map(|pattern_channel| (pattern_channel, Channel::new()))
            .collect_vec();

        for current_line in 0..self.patterns.channel_len as usize {
            for (lines, channel) in &mut channels {
                let line = &lines[current_line];
                channel.setup_line(line);
                channel.collect_signal(&mut line_audio_buffer);
                master.mix(&line_audio_buffer);
            }
            debug_assert_eq!(master.output.duration(), line_audio_buffer.duration());
            player.queue_signal(&master.output);
            master.reset();
        }

        player.play()?;

        self.playback_state = Some(Playback { player });
        self.set_poll_mode();
        Ok(())
    }

    pub fn play(&mut self) {
        if let Some(mut current_playback) = self.playback_state.take() {
            self.set_wait_mode();
            info!("Stopping playback");
            if let Err(err) = current_playback.player.stop() {
                warn!("Failed to stop previous playback: {err}");
            }
        } else if let Err(err) = self.setup_playback() {
            error!("Could not start playback: {err}");
        }
    }
}
