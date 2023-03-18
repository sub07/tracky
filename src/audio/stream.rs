use std::cmp::min;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use anyhow::{anyhow, bail, Error};
use cpal::{DefaultStreamConfigError, Device, SampleRate, SupportedStreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rust_utils_macro::New;

use crate::audio::resample;
use crate::audio::sound::Sound;
use crate::audio::value_object::Volume;

#[derive(New, Default)]
struct StreamData {
    volume_left: Volume,
    volume_right: Volume,
    #[new_default]
    queue: VecDeque<QueueBuffer>,
}

#[derive(New)]
struct QueueBuffer {
    samples: Vec<f32>,
    #[new_default]
    cursor: usize,
}

impl QueueBuffer {
    pub fn from_sound(sound: &Sound) -> QueueBuffer {
        QueueBuffer::new(sound.samples.clone())
    }
}

enum StreamCommand {
    Pause,
    Play,
    Stop,
}

#[derive(Debug, PartialEq)]
enum StreamCreationMessage {
    Info { sample_rate: u32, nb_channel: u32 },
    Done,
}

pub struct AudioStream {
    pub sample_rate: f64,
    pub nb_channel: u32,
    stream_data: Arc<Mutex<StreamData>>,
    stream_thread: JoinHandle<()>,
    stream_commands_sender: Sender<StreamCommand>,
}

impl AudioStream {
    pub fn new() -> anyhow::Result<AudioStream> {
        let stream_data = Arc::new(Mutex::new(StreamData::default()));
        let stream_data_clone = stream_data.clone();

        let (stream_commands_sender, stream_commands_receiver) = channel();
        let (stream_creation_sender, stream_creation_receiver) = channel();

        let stream_command_sender_stream_thread = stream_commands_sender.clone();

        let stream_thread = thread::spawn(move || {
            let host = cpal::default_host();

            let device = match host.default_output_device().ok_or(anyhow!("Could not find default output device")) {
                Ok(device) => device,
                Err(e) => {
                    stream_creation_sender.send(Err(e)).unwrap();
                    return;
                }
            };

            let config = match device.default_output_config().map_err(|e| anyhow!("Could not get default output config: {e:?}")) {
                Ok(config) => config,
                Err(e) => {
                    stream_creation_sender.send(Err(e)).unwrap();
                    return;
                }
            };

            let SampleRate(sample_rate) = config.sample_rate();
            let nb_channel = config.channels() as u32;

            stream_creation_sender.send(Ok(StreamCreationMessage::Info { sample_rate, nb_channel })).unwrap();

            let error_callback = |err| eprintln!("An error occurred on audio stream: {}", err);
            match device.build_output_stream(
                &config.into(),
                move |mut data: &mut [f32], _| {
                    let mut stream_data = stream_data_clone.lock().unwrap();

                    if stream_data.queue.len() == 0 {
                        stream_command_sender_stream_thread.send(StreamCommand::Pause).unwrap();
                        return;
                    }
                    let mut rem = data.len();
                    let mut out = &mut data[..];
                    while rem > 0 {
                        let buffer = stream_data.queue.front_mut().unwrap();
                        let copy_size = min(rem, buffer.samples.len() - buffer.cursor);
                        out[..copy_size].copy_from_slice(&buffer.samples[buffer.cursor..buffer.cursor + copy_size]);
                        out = &mut out[..copy_size];
                        rem -= copy_size;
                        buffer.cursor += copy_size;
                        if buffer.cursor == buffer.samples.len() - 1 {
                            stream_data.queue.pop_front();
                        }
                    }

                    for sample in data.chunks_mut(2) {
                        sample[0] *= stream_data.volume_left.value();
                        sample[1] *= stream_data.volume_right.value();
                    }
                },
                error_callback,
                None,
            ).map_err(|e| anyhow!("Failed to create output stream {e:?}")) {
                Ok(stream) => {
                    stream_creation_sender.send(Ok(StreamCreationMessage::Done)).unwrap();
                    while let Ok(command) = stream_commands_receiver.recv() {
                        match command {
                            StreamCommand::Pause => stream.pause().unwrap(),
                            StreamCommand::Stop => break,
                            StreamCommand::Play => stream.play().unwrap(),
                        };
                    }
                }
                Err(e) => {
                    stream_creation_sender.send(Err(e)).unwrap();
                }
            }
        });

        let message = stream_creation_receiver.recv()??;
        let (sample_rate, nb_channel) = if let StreamCreationMessage::Info { sample_rate, nb_channel } = message {
            (sample_rate, nb_channel)
        } else {
            bail!("Stream infos were not returned: {message:?}");
        };


        let message = stream_creation_receiver.recv()??;

        if message != StreamCreationMessage::Done {
            bail!("Stream done message was not returned: {message:?}");
        };

        Ok(AudioStream {
            sample_rate: sample_rate as f64,
            nb_channel,
            stream_commands_sender,
            stream_data,
            stream_thread,
        })
    }

    pub fn add_sound(&mut self, sound: &Sound) {
        let resampled = if sound.speed == self.sample_rate { None } else {
            Some(resample(sound, self.sample_rate))
        };
        self.stream_data.lock().unwrap().queue.push_back(QueueBuffer::from_sound(resampled.as_ref().get_or_insert(sound)));
        self.stream_commands_sender.send(StreamCommand::Play).unwrap();
    }
}

impl Drop for AudioStream {
    fn drop(&mut self) {
        self.stream_commands_sender.send(StreamCommand::Stop).unwrap();
    }
}
