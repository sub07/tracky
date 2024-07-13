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
    traits::{DeviceTrait, StreamTrait},
    Device, SampleFormat, SampleRate, Stream,
};
use eyre::ensure;
use itertools::Itertools;

use crate::log::{info, DebugLogExt};

use super::{frame::StereoFrame, signal::StereoSignal, Pan, Volume};

#[derive(Default)]
struct StreamState {
    volume: Volume,
    pan: Pan,
    pending_frames: VecDeque<StereoFrame>,
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
        Self::with_device(crate::audio::Devices::default_output()?)
    }

    pub fn with_device(device: crate::audio::Device) -> eyre::Result<Self> {
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

        info(
            "AudioPlayer",
            &format!("Playing on {device_name} at {}Hz", sample_rate.0),
        );

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

    pub fn stop(&mut self) -> eyre::Result<()> {
        self.stream_command_sender.send(StreamCommand::Stop)?;
        self.stream_state_mut().pending_frames.clear();
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

    pub fn is_playing(&self) -> bool {
        !self.stream_state().pending_frames.is_empty()
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

    for ([left_out, right_out], [left_in, right_in]) in out
        .array_chunks_mut()
        .zip(inserted_frames.map_into::<[f32; 2]>())
    {
        *left_out = left_in * left_amp.value();
        *right_out = right_in * right_amp.value();
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

#[cfg(test)]
mod test {
    use joy_vector::Vector;

    use super::*;

    const AVERAGE_SAMPLE_BUFFER_SIZE: usize = 256;
    const FLOAT_EQ_EPSILON: f32 = 0.001;

    fn simulate_signal_playing(
        volume: Volume,
        pan: Pan,
        signal: &StereoSignal,
        sample_buffer_size: usize,
    ) -> Vec<f32> {
        assert!(
            sample_buffer_size & 1 == 0,
            "Unit test internal bug, sample_buffer_size must be even"
        );
        let mut output = Vec::with_capacity(signal.len());
        let pending_frames = VecDeque::from_iter(signal.frames.iter().cloned());
        let mut stream_state = StreamState {
            volume,
            pan,
            pending_frames,
        };
        while !stream_state.pending_frames.is_empty() {
            let sample_slice_start = output.len();
            output.resize(sample_slice_start + sample_buffer_size, 0.0);
            audio_buffer_loop(&mut output[sample_slice_start..], &mut stream_state);
        }

        while output.last().is_some_and(|s| *s == 0.0) {
            output.pop();
        }

        output
    }

    fn assert_signal_eq(signal1: &[f32], signal2: &[f32]) {
        assert!(signal1
            .iter()
            .zip(signal2.iter())
            .all(|(a, b)| (a - b).abs() < FLOAT_EQ_EPSILON && !a.is_nan() && !b.is_nan()));
    }

    fn alter_signal<F>(mut signal: StereoSignal, f: F) -> StereoSignal
    where
        F: FnOnce(&mut Vec<StereoFrame>),
    {
        f(&mut signal.frames);
        signal
    }

    fn get_signal() -> StereoSignal {
        StereoSignal::from_path("assets/stereo.wav").expect("could not load asset")
    }

    #[test]
    fn test_no_pan_max_volume() {
        let signal = get_signal();
        let simulated_samples = simulate_signal_playing(
            Volume::DEFAULT,
            Pan::DEFAULT,
            &signal,
            AVERAGE_SAMPLE_BUFFER_SIZE,
        );
        assert_signal_eq(&simulated_samples, unsafe { &signal.into_samples() });
    }

    #[test]
    fn test_volume() {
        let signal = get_signal();
        const VOLUME: Volume = Volume::new_unchecked(0.1);
        let simulated_samples =
            simulate_signal_playing(VOLUME, Pan::DEFAULT, &signal, AVERAGE_SAMPLE_BUFFER_SIZE);

        let altered_signal = alter_signal(signal, |frames| {
            for Vector([left, right]) in frames.iter_mut() {
                *left *= VOLUME.value();
                *right *= VOLUME.value();
            }
        });

        assert_signal_eq(&simulated_samples, unsafe {
            &altered_signal.into_samples()
        });
    }

    #[test]
    fn test_right_pan() {
        let signal = get_signal();
        const PAN: Pan = Pan::new_unchecked(0.4);
        let simulated_samples =
            simulate_signal_playing(Volume::DEFAULT, PAN, &signal, AVERAGE_SAMPLE_BUFFER_SIZE);

        let altered_signal = alter_signal(signal, |frames| {
            for Vector([left, right]) in frames.iter_mut() {
                *left *= 0.6;
                *right *= 1.0;
            }
        });

        assert_signal_eq(&simulated_samples, unsafe {
            &altered_signal.into_samples()
        });
    }

    #[test]
    fn test_left_pan() {
        let signal = get_signal();
        const PAN: Pan = Pan::new_unchecked(-0.4);
        let simulated_samples =
            simulate_signal_playing(Volume::DEFAULT, PAN, &signal, AVERAGE_SAMPLE_BUFFER_SIZE);

        let altered_signal = alter_signal(signal, |frames| {
            for Vector([left, right]) in frames.iter_mut() {
                *left *= 1.0;
                *right *= 0.6;
            }
        });

        assert_signal_eq(&simulated_samples, unsafe {
            &altered_signal.into_samples()
        });
    }

    #[test]
    fn test_volume_pan() {
        let signal = get_signal();
        const PAN: Pan = Pan::new_unchecked(-0.4);
        const VOLUME: Volume = Volume::new_unchecked(0.1);
        let simulated_samples =
            simulate_signal_playing(VOLUME, PAN, &signal, AVERAGE_SAMPLE_BUFFER_SIZE);

        let altered_signal = alter_signal(signal, |frames| {
            for Vector([left, right]) in frames.iter_mut() {
                *left *= VOLUME.value();
                *right *= 0.6 * VOLUME.value();
            }
        });

        assert_signal_eq(&simulated_samples, unsafe {
            &altered_signal.into_samples()
        });
    }
}
