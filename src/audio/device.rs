use cpal::{
    traits::{DeviceTrait, HostTrait},
    SampleRate, SupportedBufferSize, SupportedStreamConfigRange, ALL_HOSTS,
};
use itertools::Itertools;
use joy_error::ResultLogExt;
use log::warn;

const BUFFER_SIZES: &[u32] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];

fn get_buffer_sizes(min: u32, max: u32) -> Option<&'static [u32]> {
    let start = match BUFFER_SIZES.binary_search(&min) {
        Ok(index) => index,
        Err(index) if index == BUFFER_SIZES.len() - 1 => return None,
        Err(index) => index,
    };

    let end = match BUFFER_SIZES.binary_search(&max) {
        Ok(index) => index,
        Err(0) => return None,
        Err(index) => index,
    };

    if start >= end {
        return None;
    }
    Some(&BUFFER_SIZES[start..end])
}

#[derive(Debug)]
pub struct Devices(pub Vec<Device>);

#[derive(Clone)]
pub struct Device {
    pub host_name: String,
    pub name: String,
    pub inner: cpal::Device,
    pub configs: Vec<Config>,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub sample_rate: u32,
    pub sample_format: cpal::SampleFormat,
    pub buffer_sizes: Option<&'static [u32]>,
}

#[derive(Clone)]
pub struct ConfiguredDevice {
    pub host_name: String,
    pub name: String,
    pub inner: cpal::Device,
    pub sample_format: cpal::SampleFormat,
    pub config: cpal::StreamConfig,
}

impl Device {
    pub fn configure(
        &self,
        buffer_size: cpal::BufferSize,
        config_index: usize,
    ) -> ConfiguredDevice {
        let config = &self.configs[config_index];
        ConfiguredDevice {
            host_name: self.host_name.clone(),
            name: self.name.clone(),
            inner: self.inner.clone(),
            sample_format: config.sample_format,
            config: cpal::StreamConfig {
                channels: 2,
                sample_rate: SampleRate(config.sample_rate),
                buffer_size,
            },
        }
    }
}

pub fn sample_format_bit_count(sample_format: cpal::SampleFormat) -> usize {
    match sample_format {
        cpal::SampleFormat::I8 => 8,
        cpal::SampleFormat::I16 => 16,
        cpal::SampleFormat::I32 => 32,
        cpal::SampleFormat::I64 => 64,
        cpal::SampleFormat::U8 => 8,
        cpal::SampleFormat::U16 => 16,
        cpal::SampleFormat::U32 => 32,
        cpal::SampleFormat::U64 => 64,
        cpal::SampleFormat::F32 => 32,
        cpal::SampleFormat::F64 => 64,
        format => panic!("Unsupported sample format: {format}"),
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("host_name", &self.host_name)
            .field("name", &self.name)
            .field("configs", &self.configs)
            .finish()
    }
}

impl std::fmt::Debug for ConfiguredDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfiguredDevice")
            .field("host_name", &self.host_name)
            .field("name", &self.name)
            .field("sample_format", &self.sample_format)
            .field("config", &self.config)
            .finish()
    }
}

fn map_config(config: SupportedStreamConfigRange) -> Option<Config> {
    if config.channels() != 2 {
        return None;
    }
    let buffer_sizes = match *config.buffer_size() {
        SupportedBufferSize::Range { min, max } => {
            if let Some(buffer_sizes) = get_buffer_sizes(min, max) {
                Some(buffer_sizes)
            } else {
                return None;
            }
        }
        SupportedBufferSize::Unknown => None,
    };
    if config.max_sample_rate() != config.min_sample_rate() {
        warn!(
            "min sample rate != max sample rate: min={} / max={}",
            config.min_sample_rate().0,
            config.max_sample_rate().0
        );
    }
    Some(Config {
        buffer_sizes,
        sample_format: config.sample_format(),
        sample_rate: config.max_sample_rate().0,
    })
}

fn map_device(host_name: String, device: cpal::Device) -> Option<Device> {
    let configs = device
        .supported_output_configs()
        .log_ok()?
        .filter_map(map_config)
        .collect_vec();

    if configs.is_empty() {
        return None;
    }

    Some(Device {
        host_name,
        name: device.name().unwrap_or("Unknown device".into()),
        inner: device,
        configs,
    })
}

pub fn default_output() -> Option<ConfiguredDevice> {
    let device = cpal::default_host()
        .default_output_device()
        .and_then(|device| map_device(cpal::default_host().id().name().to_string(), device))?;
    let config = device.inner.default_output_config().log_ok()?;
    Some(ConfiguredDevice {
        host_name: device.host_name,
        name: device.name,
        inner: device.inner,
        sample_format: config.sample_format(),
        config: config.into(),
    })
}

impl Devices {
    pub fn load() -> Devices {
        Devices(
            ALL_HOSTS
                .iter()
                .filter_map(|host_id| {
                    cpal::host_from_id(*host_id).log_ok().and_then(|host| {
                        host.output_devices().log_ok().map(|devices| {
                            devices.map(|device| (host_id.name().to_string(), device))
                        })
                    })
                })
                .flatten()
                .filter_map(|(host_name, device)| map_device(host_name, device))
                .collect(),
        )
    }
}
