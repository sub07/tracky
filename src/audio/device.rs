use std::{collections::HashMap, ops};

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

impl Devices {
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
                                        .map(|device| {
                                            Device(
                                                device
                                                    .name()
                                                    .unwrap_or("Unknown device name".into()),
                                                device,
                                            )
                                        })
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
