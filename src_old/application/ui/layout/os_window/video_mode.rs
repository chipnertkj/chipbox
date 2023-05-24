use std::error::Error;
use std::fmt::Display;

use crate::config;
use winit::monitor;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct VideoModeSerialized {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub bit_depth: u16,
    pub refresh_rate_millihertz: u32,
}

impl std::cmp::PartialEq<monitor::VideoMode> for VideoModeSerialized {
    fn eq(&self, other: &monitor::VideoMode) -> bool {
        self.size == other.size()
            && self.bit_depth == other.bit_depth()
            && self.refresh_rate_millihertz == other.refresh_rate_millihertz()
    }
}

#[derive(Debug)]
pub enum VideoModeDeserializationError {
    NoMatch,
}

impl Display for VideoModeDeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NoMatch => "no matching video mode found",
        })
    }
}

impl Error for VideoModeDeserializationError {}

impl config::SerializedItemTrait<monitor::VideoMode, &monitor::MonitorHandle>
    for VideoModeSerialized
{
    type SerializationError = !;
    type DeserializationError = VideoModeDeserializationError;

    fn serialize(
        value: monitor::VideoMode,
    ) -> Result<Self, Self::SerializationError> {
        Ok(Self {
            size: value.size(),
            bit_depth: value.bit_depth(),
            refresh_rate_millihertz: value.refresh_rate_millihertz(),
        })
    }

    /// Returns Err(()) if no matching video mode is found.
    fn deserialize(
        &self,
        handle: &monitor::MonitorHandle,
    ) -> Result<monitor::VideoMode, Self::DeserializationError> {
        let video_modes = handle.video_modes();
        for mode in video_modes {
            if *self == mode {
                return Ok(mode);
            }
        }
        Err(Self::DeserializationError::NoMatch)
    }
}
