use cpal::traits::{DeviceTrait, HostTrait};
use iter_tools::Itertools;

pub fn devices() -> anyhow::Result<Vec<cpal::Device>> {
    Ok(cpal::default_host().output_devices()?.collect_vec())
}

pub fn stereo_devices() -> anyhow::Result<Vec<cpal::Device>> {
    Ok(devices()?
        .into_iter()
        .filter_map(|d| {
            if let Ok(config) = d.default_output_config() {
                if config.channels() == 2 {
                    Some(d)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect_vec())
}
