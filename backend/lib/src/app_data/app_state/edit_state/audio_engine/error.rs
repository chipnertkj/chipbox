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
            SettingsError::StreamConfigParse(err) => Some(err),
            SettingsError::HostIdParse(err) => Some(err),
            SettingsError::InvalidStreamConfig(err) => Some(err),
        }
    }
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::StreamConfigParse(err) => err.fmt(f),
            SettingsError::HostIdParse(err) => err.fmt(f),
            SettingsError::InvalidStreamConfig(err) => err.fmt(f),
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
            ResetStreamError::Config(err) => Some(err),
            ResetStreamError::Play(err) => Some(err),
        }
    }
}

impl std::fmt::Display for ResetStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResetStreamError::Config(err) => err.fmt(f),
            ResetStreamError::Play(err) => err.fmt(f),
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
            ResetDeviceError::Stream(err) => Some(err),
            ResetDeviceError::Device(err) => Some(err),
        }
    }
}

impl std::fmt::Display for ResetDeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResetDeviceError::Stream(err) => err.fmt(f),
            ResetDeviceError::Device(err) => err.fmt(f),
        }
    }
}
