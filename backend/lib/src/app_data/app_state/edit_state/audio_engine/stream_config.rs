pub use sample_format::SampleFormat;

use chipbox_common as common;
use std::str::FromStr as _;
mod sample_format;

#[derive(Debug)]
pub enum Error {
    Device(super::device::Error),
    NoMatchingConfig,
    UnsupportedChannelCount(u16),
    Other(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Device(err) => write!(f, "{err}"),
            Error::NoMatchingConfig => {
                write!(
                    f,
                    "unable to find a matching stream config for this device"
                )
            }
            Error::UnsupportedChannelCount(err) => {
                write!(f, "unsupported channel count: {err}")
            }
            Error::Other(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Device(err) => Some(err),
            Error::NoMatchingConfig => None,
            Error::UnsupportedChannelCount(_) => None,
            Error::Other(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum StreamConfig {
    #[default]
    Default,
    Custom {
        sample_format: SampleFormat,
        sample_rate: cpal::SampleRate,
        channels: cpal::ChannelCount,
    },
}

impl StreamConfig {
    pub fn from_settings(
        settings: &common::audio_engine::StreamConfig,
    ) -> Result<Self, ParseError> {
        Self::try_from(settings)
    }
}

#[derive(Debug)]
pub enum ParseError {
    SampleFormat(sample_format::ParseError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::SampleFormat(err) => {
                write!(f, "unable to parse sample format: {err}")
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::SampleFormat(err) => Some(err),
        }
    }
}

impl TryFrom<&common::audio_engine::StreamConfig> for StreamConfig {
    type Error = ParseError;
    fn try_from(
        value: &common::audio_engine::StreamConfig,
    ) -> Result<Self, Self::Error> {
        match value {
            common::audio_engine::StreamConfig::Default => Ok(Self::Default),
            common::audio_engine::StreamConfig::Custom {
                sample_format,
                sample_rate,
                channels,
                buffer_duration: _,
            } => {
                let sample_format = SampleFormat::from_str(sample_format)
                    .map_err(ParseError::SampleFormat)?;
                Ok(Self::Custom {
                    sample_format,
                    sample_rate: cpal::SampleRate(*sample_rate),
                    channels: *channels,
                })
            }
        }
    }
}
