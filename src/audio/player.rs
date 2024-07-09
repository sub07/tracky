use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, SampleFormat, SampleRate, Stream,
};
use eyre::{ensure, OptionExt};

use crate::{
    log::{info, DebugLogExt},
    DEBUG,
};

use super::{frame::StereoFrame, signal::StereoSignal, Pan, Volume};

#[derive(Default)]
struct StreamState {
    volume: Volume,
    pan: Pan,
    pending_frames: VecDeque<StereoFrame>,
    // If the Option is Some, then the sink is filled with the same data that are sent to audio driver
    debug_sample_sink: Option<Vec<f32>>,
}

enum StreamCommand {
    Play,
    Stop,
    Pause,
}

pub struct Player {
    pub sample_rate: f32,
    stream_command_sender: Sender<StreamCommand>,
    stream_state: Arc<Mutex<StreamState>>,
}

impl Player {
    pub fn with_default_device() -> eyre::Result<Self> {
        let default_device = default_host()
            .default_output_device()
            .ok_or_eyre("Could not get default output device")?;
        Self::with_device(default_device)
    }

    pub fn with_device(device: Device) -> eyre::Result<Self> {
        let (stream_creation_sender, stream_creation_receiver) = mpsc::channel();
        let (stream_command_sender, stream_command_receiver) = mpsc::channel();

        let stream_state = Arc::new(Mutex::new(StreamState {
            debug_sample_sink: if DEBUG { Some(Vec::new()) } else { None },
            ..Default::default()
        }));
        let audio_thread_stream_state = stream_state.clone();

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
                                        err.error("play");
                                        break;
                                    }
                                }
                                StreamCommand::Stop => break,
                                StreamCommand::Pause => {
                                    if let Err(err) = stream.pause() {
                                        err.error("pause");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => stream_creation_sender.send(Err(e)).unwrap(),
                },
            )?;

        let sample_rate = stream_creation_receiver.recv()??;

        info("player", &"Created");
        info("player", &format!("Sample rate: {}", sample_rate.0));

        Ok(Player {
            sample_rate: sample_rate.0 as f32,
            stream_command_sender,
            stream_state,
        })
    }

    pub fn play(&self) -> eyre::Result<()> {
        self.stream_command_sender.send(StreamCommand::Play)?;
        Ok(())
    }

    pub fn pause(&self) -> eyre::Result<()> {
        self.stream_command_sender.send(StreamCommand::Pause)?;
        Ok(())
    }

    pub fn stop(&self) -> eyre::Result<()> {
        self.stream_command_sender.send(StreamCommand::Stop)?;
        Ok(())
    }

    pub fn set_volume(&mut self, v: Volume) {
        self.stream_state_mut().volume = v;
    }

    pub fn set_pan(&mut self, p: Pan) {
        self.stream_state_mut().pan = p;
    }

    pub fn volume(&self) -> Volume {
        self.stream_state().volume
    }

    pub fn pan(&self) -> Pan {
        self.stream_state().pan
    }

    fn stream_state_mut(&mut self) -> impl DerefMut<Target = StreamState> + '_ {
        self.stream_state.lock().unwrap()
    }

    fn stream_state(&self) -> impl Deref<Target = StreamState> + '_ {
        self.stream_state.lock().unwrap()
    }

    pub fn queue_signal(&mut self, signal: &StereoSignal) {
        self.stream_state_mut().pending_frames.extend(signal.iter());
    }
}

fn init_stream(
    device: Device,
    stream_state: Arc<Mutex<StreamState>>,
) -> eyre::Result<(Stream, SampleRate)> {
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
        |e| {
            e.debug("player stream error");
        },
        None,
    )?;

    Ok((stream, sample_rate))
}

fn audio_buffer_loop(out: &mut [f32], mut stream_state: impl DerefMut<Target = StreamState>) {
    let left_amp = stream_state.pan.left_volume() * stream_state.volume;
    let right_amp = stream_state.pan.right_volume() * stream_state.volume;

    let available_frame_count = out.len() / 2;
    let inserted_frame_count = available_frame_count.min(stream_state.pending_frames.len());
    let inserted_frames = stream_state.pending_frames.drain(..inserted_frame_count);

    for ([left_out, right_out], (left_in, right_in)) in out.array_chunks_mut().zip(inserted_frames)
    {
        *left_out = left_in * left_amp.value();
        *right_out = right_in * right_amp.value();
    }

    if inserted_frame_count < available_frame_count {
        let start_index = inserted_frame_count * 2;
        out[start_index..].fill(0.0);
    }

    if let Some(ref mut sink) = stream_state.debug_sample_sink {
        for s in out {
            sink.push(*s);
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use thread::sleep;

    use super::*;

    fn get_player() -> Player {
        Player::with_default_device().expect("Cannot build player for unit tests")
    }

    fn assert_that_player_played_signal(player: Player, signal: StereoSignal) {
        let _ = player.stop();
        while Arc::strong_count(&player.stream_state) > 1 {
            sleep(Duration::from_millis(100));
        }
        let played_samples = Arc::into_inner(player.stream_state)
            .unwrap()
            .into_inner()
            .unwrap()
            .debug_sample_sink
            .expect("Use a debug player in unit tests");

        assert!(played_samples
            .into_iter()
            .zip(signal.into_samples())
            .all(|(a, b)| a == b && !a.is_nan() && !b.is_nan()));
    }

    fn wait_player_done_playing(player: &Player) {
        while !player.stream_state().pending_frames.is_empty() {}
    }

    fn alter_signal<F>(mut signal: StereoSignal, f: F) -> StereoSignal
    where
        F: FnOnce(&mut Vec<StereoFrame>),
    {
        f(&mut signal.frames);
        signal
    }

    #[test]
    fn test_no_pan_max_volume() {
        let mut player = get_player();

        let signal = StereoSignal::from_path("assets/piano.wav").expect("could not load asset");
        player.queue_signal(&signal);
        let _ = player.play();

        wait_player_done_playing(&player);
        assert_that_player_played_signal(player, signal);
    }

    #[test]
    fn test_volume() {
        let mut player = get_player();

        let signal = StereoSignal::from_path("assets/piano.wav").expect("could not load asset");
        player.queue_signal(&signal);

        const VOLUME: Volume = Volume::new_unchecked(0.1);

        player.set_volume(VOLUME);
        let _ = player.play();

        wait_player_done_playing(&player);

        let altered_signal = alter_signal(signal, |frames| {
            for (left, right) in frames.iter_mut() {
                *left *= VOLUME.value();
                *right *= VOLUME.value();
            }
        });

        assert_that_player_played_signal(player, altered_signal);
    }

    #[test]
    fn test_right_pan() {
        let mut player = get_player();

        let signal = StereoSignal::from_path("assets/piano.wav").expect("could not load asset");
        player.queue_signal(&signal);

        const PAN: Pan = Pan::new_unchecked(0.5);

        player.set_pan(PAN);
        let _ = player.play();

        wait_player_done_playing(&player);

        let altered_signal = alter_signal(signal, |frames| {
            for (left, right) in frames.iter_mut() {
                *left *= 0.5;
                *right *= 1.0;
            }
        });

        assert_that_player_played_signal(player, altered_signal);
    }

    #[test]
    fn test_left_pan() {
        let mut player = get_player();

        let signal = StereoSignal::from_path("assets/piano.wav").expect("could not load asset");
        player.queue_signal(&signal);

        const PAN: Pan = Pan::new_unchecked(-0.5);

        player.set_pan(PAN);
        let _ = player.play();

        wait_player_done_playing(&player);

        let altered_signal = alter_signal(signal, |frames| {
            for (left, right) in frames.iter_mut() {
                *left *= 1.0;
                *right *= 0.5;
            }
        });

        assert_that_player_played_signal(player, altered_signal);
    }

    #[test]
    fn test_volume_left_pan() {
        let mut player = get_player();

        let signal = StereoSignal::from_path("assets/piano.wav").expect("could not load asset");
        player.queue_signal(&signal);

        const PAN: Pan = Pan::new_unchecked(-0.5);
        const VOLUME: Volume = Volume::new_unchecked(0.1);

        player.set_pan(PAN);
        player.set_volume(VOLUME);
        let _ = player.play();

        wait_player_done_playing(&player);

        let altered_signal = alter_signal(signal, |frames| {
            for (left, right) in frames.iter_mut() {
                *left *= VOLUME.value();
                *right *= 0.5 * VOLUME.value();
            }
        });

        assert_that_player_played_signal(player, altered_signal);
    }
}
