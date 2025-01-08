use std::fmt::Display;

use anyhow::Context;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    BufferSize, FrameCount, SampleFormat, SampleRate, SupportedStreamConfig,
    SupportedStreamConfigRange, ALL_HOSTS,
};
use itertools::Itertools;
use joy_error::ResultLogExt;
use log::debug;

#[derive(Debug)]
pub struct Hosts(pub Vec<Host>);

#[derive(Debug)]
pub struct Host {
    pub name: String,
    pub devices: Vec<Device>,
}

#[derive(Clone)]
pub struct Device {
    pub name: String,
    pub inner: cpal::Device,
    pub config: SupportedStreamConfig,
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("name", &self.name)
            .field("config", &self.config)
            .finish()
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn map_device(device: cpal::Device) -> Option<Device> {
    find_first_valid_config(&device).map(|config| Device {
        name: device.name().unwrap_or("Unknown device".into()),
        inner: device,
        config,
    })
}

pub fn default_output() -> anyhow::Result<Device> {
    Ok(cpal::default_host()
        .default_output_device()
        .and_then(map_device)
        .context("Could not get default output device")?)
}

pub fn find_first_valid_config(device: &cpal::Device) -> Option<SupportedStreamConfig> {
    fn is_sample_rate_valid(config: SupportedStreamConfigRange, sample_rate: u32) -> bool {
        sample_rate >= config.min_sample_rate().0 && sample_rate <= config.max_sample_rate().0
    }

    device
        .supported_output_configs()
        .log_ok()
        .map(|mut configs| {
            configs
                .filter(|config| config.channels() == 2)
                .map(|config| {
                    (
                        config,
                        if is_sample_rate_valid(config, 44100) {
                            44100
                        } else {
                            0
                        },
                    )
                })
                .max_by_key(|(_, score)| *score)
        })
        .flatten()
        .map(|(config, score)| {
            if score == 44100 {
                config.with_sample_rate(SampleRate(44100))
            } else {
                config.with_max_sample_rate()
            }
        })
}

impl Hosts {
    pub fn load() -> Hosts {
        Hosts(
            ALL_HOSTS
                .iter()
                .filter_map(|host_id| {
                    cpal::host_from_id(*host_id)
                        .ok()
                        .and_then(|host| {
                            host.output_devices()
                                .map(|devices| Host {
                                    name: host.id().name().to_owned(),
                                    devices: devices.filter_map(map_device).collect_vec(),
                                })
                                .ok()
                        })
                        // We filter out hosts with no device
                        .filter(|host| !host.devices.is_empty())
                })
                .collect(),
        )
    }
}
