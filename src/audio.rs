use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, SupportedStreamConfig,
};

/// Gets the default audio host.
pub fn get_default_host() -> cpal::Host {
    cpal::default_host()
}

/// Gets the default output device from the given host.
pub fn get_default_output_device(host: &cpal::Host) -> Result<Device, Box<dyn std::error::Error>> {
    host.default_output_device()
        .ok_or_else(|| "no output device available".into())
}

/// Gets the default output configuration for the given device.
pub fn get_default_output_config(
    device: &Device,
) -> Result<SupportedStreamConfig, Box<dyn std::error::Error>> {
    device.default_output_config().map_err(|e| e.into())
}
