use anyhow::Context;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    SampleFormat, ALL_HOSTS,
};
use itertools::Itertools;

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
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn map_device_name(device: &cpal::Device) -> String {
    device.name().unwrap_or("Unknown device".into())
}

pub fn default_output() -> anyhow::Result<Device> {
    let default_device = cpal::default_host()
        .default_output_device()
        .context("Could not get default output device")?;
    Ok(Device {
        name: map_device_name(&default_device),
        inner: default_device,
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
                                    devices: devices
                                        .filter(|device| {
                                            // We take default config for now, only stereo devices that uses f32
                                            device.default_output_config().is_ok_and(|config| {
                                                config.channels() == 2
                                                    && config.sample_format() == SampleFormat::F32
                                            })
                                        })
                                        .map(|device| Device {
                                            name: map_device_name(&device),
                                            inner: device,
                                        })
                                        .collect_vec(),
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
