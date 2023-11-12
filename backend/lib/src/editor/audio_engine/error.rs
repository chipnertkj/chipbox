use super::{device, host_id, stream_config};

#[derive(Debug)]
pub enum SettingsError {
    StreamConfigParse(stream_config::ParseError),
    HostIdParse(host_id::ParseError),
    InvalidStreamConfig(stream_config::Error),
}

impl std::error::Error for SettingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SettingsError::StreamConfigParse(e) => Some(e),
            SettingsError::HostIdParse(e) => Some(e),
            SettingsError::InvalidStreamConfig(e) => Some(e),
        }
    }
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::StreamConfigParse(e) => e.fmt(f),
            SettingsError::HostIdParse(e) => e.fmt(f),
            SettingsError::InvalidStreamConfig(e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum ResetStreamError {
    Config(stream_config::Error),
    Play(cpal::PlayStreamError),
}

impl std::error::Error for ResetStreamError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ResetStreamError::Config(e) => Some(e),
            ResetStreamError::Play(e) => Some(e),
        }
    }
}

impl std::fmt::Display for ResetStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResetStreamError::Config(e) => e.fmt(f),
            ResetStreamError::Play(e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum ResetDeviceError {
    Stream(ResetStreamError),
    Device(device::Error),
}

impl std::error::Error for ResetDeviceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ResetDeviceError::Stream(e) => Some(e),
            ResetDeviceError::Device(e) => Some(e),
        }
    }
}

impl std::fmt::Display for ResetDeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResetDeviceError::Stream(e) => e.fmt(f),
            ResetDeviceError::Device(e) => e.fmt(f),
        }
    }
}
