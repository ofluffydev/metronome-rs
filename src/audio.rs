use cpal::{
    Device, SupportedStreamConfig,
    traits::{DeviceTrait, HostTrait},
};

/// Gets the default audio host.
#[must_use]
pub fn get_default_host() -> cpal::Host {
    cpal::default_host()
}

/// Gets the default output device from the given host.
///
/// # Errors
///
/// Returns an error if no output device is available on the host.
pub fn get_default_output_device(host: &cpal::Host) -> Result<Device, Box<dyn std::error::Error>> {
    host.default_output_device()
        .ok_or_else(|| "no output device available".into())
}

/// Gets the default output configuration for the given device.
///
/// # Errors
///
/// Returns an error if the default output configuration cannot be retrieved from the device.
pub fn get_default_output_config(
    device: &Device,
) -> Result<SupportedStreamConfig, Box<dyn std::error::Error>> {
    device
        .default_output_config()
        .map_err(std::convert::Into::into)
}
