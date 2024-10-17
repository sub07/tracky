use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use anyhow::ensure;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, SampleFormat, SampleRate, Stream,
};

use itertools::Itertools;
use log::{error, info};

use crate::model::pattern::Patterns;

use super::{frame::StereoFrame, signal::StereoSignal, Pan, Volume};

pub struct PatternsPlayback {
    pub player: Player,
    channels: Vec<Channel>,
    line_audio_buffer: StereoSignal,
    current_line: usize,
    line_duration: Duration,
    time_since_last_line: Duration,
    sink: StereoSignal,
}

struct PlayerState {
    volume: Volume,
    pan: Pan,
    patterns: Arc<Mutex<Patterns>>,
    playback: Arc<Mutex<PatternsPlayback>>
}

enum StreamCommand {
    Play,
    Stop,
}

pub struct Player {
    pub frame_rate: f32,
    stream_command_sender: Sender<StreamCommand>,
    stream_state: Arc<Mutex<PlayerState>>,
}

impl Player {
    pub fn with_default_device() -> anyhow::Result<Self> {
        Self::with_device(crate::audio::Devices::default_output()?)
    }

    pub fn with_device(device: crate::audio::Device) -> anyhow::Result<Self> {
        let (stream_creation_sender, stream_creation_receiver) = mpsc::channel();
        let (stream_command_sender, stream_command_receiver) = mpsc::channel();

        let stream_state = Arc::new(Mutex::new(Default::default()));
        let audio_thread_stream_state = stream_state.clone();

        let crate::audio::device::Device(device_name, device) = device;

        thread::Builder::new()
            .name("audio_commands_handler".into())
            .spawn(
                move || match init_stream(device, audio_thread_stream_state) {
                    Ok((stream, sample_rate)) => {
                        stream_creation_sender.send(Ok(sample_rate)).unwrap();
                        while let Ok(command) = stream_command_receiver.recv() {
                            match command {
                                StreamCommand::Play => {
                                    if let Err(err) = stream.play() {
                                        error!("{err:?} when start playback");
                                        break;
                                    }
                                }
                                StreamCommand::Stop => break,
                            }
                        }
                    }
                    Err(e) => stream_creation_sender.send(Err(e)).unwrap(),
                },
            )?;

        let sample_rate = stream_creation_receiver.recv()??;

        info!("Playing on {device_name} at {}Hz", sample_rate.0);

        let player = Player {
            frame_rate: sample_rate.0 as f32,
            stream_command_sender,
            stream_state,
        };

        player.play()?;

        Ok(player)
    }

    pub fn play(&self) -> anyhow::Result<()> {
        self.stream_command_sender.send(StreamCommand::Play)?;
        Ok(())
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        self.stream_command_sender.send(StreamCommand::Stop)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_volume(&mut self, v: Volume) {
        self.stream_state_mut().volume = v;
    }
    #[allow(dead_code)]

    pub fn set_pan(&mut self, p: Pan) {
        self.stream_state_mut().pan = p;
    }
    #[allow(dead_code)]

    pub fn volume(&self) -> Volume {
        self.stream_state().volume
    }
    #[allow(dead_code)]

    pub fn pan(&self) -> Pan {
        self.stream_state().pan
    }

    fn stream_state_mut(&mut self) -> impl DerefMut<Target = PlayerState> + '_ {
        self.stream_state.lock().unwrap()
    }

    fn stream_state(&self) -> impl Deref<Target = PlayerState> + '_ {
        self.stream_state.lock().unwrap()
    }
}

fn init_stream(
    device: Device,
    stream_state: Arc<Mutex<PlayerState>>,
) -> anyhow::Result<(Stream, SampleRate)> {
    let config = device.default_output_config()?;

    let sample_rate = config.sample_rate();

    ensure!(config.channels() == 2);
    ensure!(config.sample_format() == SampleFormat::F32);

    let stream = device.build_output_stream(
        &config.into(),
        move |out: &mut [f32], _| {
            let lock = stream_state.lock().unwrap();
            audio_buffer_loop(out, lock);
        },
        |e| error!("Cannot start audio stream: {e:?}"),
        None,
    )?;

    Ok((stream, sample_rate))
}

fn audio_buffer_loop(out: &mut [f32], mut stream_state: impl DerefMut<Target = PlayerState>) {
    //todo zero output

    let left_amp = stream_state.pan.left_volume() * stream_state.volume;
    let right_amp = stream_state.pan.right_volume() * stream_state.volume;

    let available_frame_count = out.len() / 2;
    let inserted_frame_count = available_frame_count.min(stream_state.pending_frames.len());
    let inserted_frames = stream_state.pending_frames.drain(..inserted_frame_count);

    for (out, [left_in, right_in]) in out
        .chunks_exact_mut(2)
        .zip(inserted_frames.map_into::<[f32; 2]>())
    {
        out[0] = left_in * left_amp.value();
        out[1] = right_in * right_amp.value();
    }

    if inserted_frame_count < available_frame_count {
        let start_index = inserted_frame_count * 2;
        out[start_index..].fill(0.0);
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        // This panics if the player still has samples queued to play.
        // Because the underlaying stream is dropped silently if the player goes out of scope, I added this assertion to help not forget to store the player somewhere after creation.
        // Maybe there are some use case but if a player must be dropped while playing, self.stop() should be called before dropping.
        // If this is problematic, let's just replace it with a simple log.
        assert!(
            !self.is_playing(),
            "Player is still playing while being dropped"
        );
    }
}
