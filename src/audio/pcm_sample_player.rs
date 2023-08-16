use std::collections::VecDeque;
use std::iter::Peekable;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::vec::IntoIter;

use anyhow::{anyhow, bail};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rust_utils_macro::New;

use super::pcm_sample::PcmStereoSample;
use super::{Pan, Volume};

#[derive(New, Default)]
struct StreamData {
    volume: Volume,
    pan: Pan,
    #[new_default]
    queue: VecDeque<Peekable<IntoIter<(f32, f32)>>>,
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

pub struct PcmSamplePlayer {
    pub sample_rate: f32,
    pub nb_channel: u32,
    stream_data: Arc<Mutex<StreamData>>,
    stream_commands_sender: Sender<StreamCommand>,
}

impl PcmSamplePlayer {
    pub fn new() -> anyhow::Result<PcmSamplePlayer> {
        let stream_data = Arc::new(Mutex::new(StreamData::default()));
        let stream_data_clone = stream_data.clone();

        let (stream_commands_sender, stream_commands_receiver) = channel();
        let (stream_creation_sender, stream_creation_receiver) = channel();

        let _stream_command_sender_stream_thread = stream_commands_sender.clone();

        thread::Builder::new()
            .name("cpal stream thread".to_string())
            .spawn(move || {
                let host = cpal::default_host();

                let device = match host
                    .default_output_device()
                    .ok_or(anyhow!("Could not find default output device"))
                {
                    Ok(device) => device,
                    Err(e) => {
                        stream_creation_sender.send(Err(e)).unwrap();
                        return;
                    }
                };

                let config = match device
                    .default_output_config()
                    .map_err(|e| anyhow!("Could not get default output config: {e:?}"))
                {
                    Ok(config) => config,
                    Err(e) => {
                        stream_creation_sender.send(Err(e)).unwrap();
                        return;
                    }
                };

                let cpal::SampleRate(sample_rate) = config.sample_rate();
                let nb_channel = config.channels() as u32;

                stream_creation_sender
                    .send(Ok(StreamCreationMessage::Info {
                        sample_rate,
                        nb_channel,
                    }))
                    .unwrap();

                let error_callback = |err| eprintln!("An error occurred on audio stream: {}", err);
                match device
                    .build_output_stream(
                        &config.into(),
                        move |data: &mut [f32], _| {
                            let mut stream_data = stream_data_clone.lock().unwrap();

                            if stream_data.queue.is_empty() {
                                // TODO Introduce delay between last samples and pause to avoid clicks
                                // stream_command_sender_stream_thread.send(StreamCommand::Pause).unwrap();
                                data.fill(0.0);
                                return;
                            }

                            let pan = stream_data.pan.value();
                            let volume = stream_data.volume.value();

                            let left_volume = (1.0 - pan.clamp(0.0, 1.0)) * volume;
                            let right_volume = (1.0 + pan.clamp(-1.0, 0.0)) * volume;

                            let mut out = data.iter_mut().peekable();

                            while out.peek().is_some() {
                                let current_iter = match stream_data.queue.front_mut() {
                                    Some(front) => front,
                                    None => return,
                                };
                                let (l, r) = current_iter.next().unwrap();
                                *out.next().unwrap() = l * left_volume;
                                *out.next().unwrap() = r * right_volume;
                                if current_iter.peek().is_none() {
                                    stream_data.queue.pop_front();
                                }
                            }
                        },
                        error_callback,
                        None,
                    )
                    .map_err(|e| anyhow!("Failed to create output stream {e:?}"))
                {
                    Ok(stream) => {
                        stream_creation_sender
                            .send(Ok(StreamCreationMessage::Done))
                            .unwrap();
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
            })?;

        let message = stream_creation_receiver.recv()??;
        let (sample_rate, nb_channel) = if let StreamCreationMessage::Info {
            sample_rate,
            nb_channel,
        } = message
        {
            (sample_rate, nb_channel)
        } else {
            bail!("Stream infos were not returned: {message:?}");
        };

        let message = stream_creation_receiver.recv()??;

        if message != StreamCreationMessage::Done {
            bail!("Stream done message was not returned: {message:?}");
        };

        Ok(PcmSamplePlayer {
            sample_rate: sample_rate as f32,
            nb_channel,
            stream_commands_sender,
            stream_data,
        })
    }

    pub fn queue_pcm_samples(&mut self, sound: &PcmStereoSample) -> anyhow::Result<()> {
        self.stream_data
            .lock()
            .unwrap()
            .queue
            .push_back(sound.frames.clone().into_iter().peekable());
        self.stream_commands_sender.send(StreamCommand::Play)?;
        Ok(())
    }

    pub fn volume(&mut self, volume: Volume) {
        let mut stream_data = self.stream_data.lock().unwrap();
        stream_data.volume = volume;
    }
}

impl Drop for PcmSamplePlayer {
    fn drop(&mut self) {
        self.stream_commands_sender
            .send(StreamCommand::Stop)
            .unwrap();
    }
}