use std::{collections::HashMap, ops};

use anyhow::Context;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    HostId, SampleFormat, ALL_HOSTS,
};
use itertools::Itertools;

pub struct Devices {
    devices: HashMap<HostId, Vec<Device>>,
}

#[derive(Clone)]
pub struct Device(pub String, pub cpal::Device);

fn map_device_name(device: &cpal::Device) -> String {
    device.name().unwrap_or("Unknown device".into())
}

impl Devices {
    pub fn default_output() -> anyhow::Result<Device> {
        let default_device = cpal::default_host()
            .default_output_device()
            .context("Could not get default output device")?;
        Ok(Device(map_device_name(&default_device), default_device))
    }

    pub fn load() -> Devices {
        Devices {
            devices: ALL_HOSTS
                .iter()
                .filter_map(|id| {
                    cpal::host_from_id(*id).ok().and_then(|host| {
                        host.output_devices()
                            .map(|devices| {
                                (
                                    host.id(),
                                    devices
                                        .filter(|device| {
                                            device.default_output_config().is_ok_and(|config| {
                                                config.channels() == 2
                                                    && config.sample_format() == SampleFormat::F32
                                            })
                                        })
                                        .map(|device| Device(map_device_name(&device), device))
                                        .collect_vec(),
                                )
                            })
                            .ok()
                    })
                })
                .collect(),
        }
    }

    pub fn hosts(&self) -> impl Iterator<Item = HostId> + '_ {
        self.devices.keys().cloned()
    }

    pub fn devices(&self, host_id: &HostId) -> Option<&[Device]> {
        self.devices.get(host_id).map(ops::Deref::deref)
    }

    pub fn host_count(&self) -> usize {
        self.devices.len()
    }
}
