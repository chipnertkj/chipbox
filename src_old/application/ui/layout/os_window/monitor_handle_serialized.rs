use std::error::Error;
use std::fmt::Display;

use winit::{event_loop, monitor};

use crate::config;

/// An approximation of a monitor handle.
/// The contained information is used to identify monitors.
///
/// `winit` does not provide any other way to do this.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MonitorHandleSerialized {
    pub name: String,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub position: winit::dpi::PhysicalPosition<i32>,
    pub refresh_rate_millihertz: u32,
    pub video_modes: Vec<super::VideoModeSerialized>,
}

impl MonitorHandleSerialized {
    /// Returns `None` if the monitor was disconnected during the comparison.
    pub fn try_eq(&self, other: &monitor::MonitorHandle) -> Option<bool> {
        let mut other_video_modes = other.video_modes();
        let mut video_modes_match = || -> bool {
            let mut do_all_match = true;
            self.video_modes
                .iter()
                .for_each(|x| {
                    if !other_video_modes.any(|y| x == &y) {
                        do_all_match = false
                    }
                });
            do_all_match
        };

        Some(
            self.name == other.name()?
                && self.size == other.size()
                && self.position == other.position()
                && self.refresh_rate_millihertz
                    == other.refresh_rate_millihertz()?
                && video_modes_match(),
        )
    }
}

#[derive(Debug)]
pub enum MonitorHandleDeserializationError {
    Disconnected,
    NoMatch,
}

impl Display for MonitorHandleDeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Disconnected => {
                "monitor was disconnected during serialization/deserialization"
            }
            Self::NoMatch => "no matching monitor found",
        })
    }
}

impl Error for MonitorHandleDeserializationError {}

impl<T>
    config::SerializedItemTrait<
        monitor::MonitorHandle,
        &event_loop::EventLoop<T>,
    > for MonitorHandleSerialized
{
    type SerializationError = MonitorHandleDeserializationError;
    type DeserializationError = MonitorHandleDeserializationError;

    fn serialize(
        value: monitor::MonitorHandle,
    ) -> Result<Self, Self::SerializationError> {
        let name = value
            .name()
            .ok_or(Self::SerializationError::Disconnected)?;
        let size = value.size();
        let position = value.position();
        let refresh_rate_millihertz = value
            .refresh_rate_millihertz()
            .ok_or(Self::SerializationError::Disconnected)?;
        let video_modes = value
            .video_modes()
            .map(|x| super::VideoModeSerialized {
                size: x.size(),
                bit_depth: x.bit_depth(),
                refresh_rate_millihertz: x.refresh_rate_millihertz(),
            })
            .collect();
        Ok(Self {
            name,
            size,
            position,
            refresh_rate_millihertz,
            video_modes,
        })
    }

    fn deserialize(
        &self,
        event_loop: &event_loop::EventLoop<T>,
    ) -> Result<monitor::MonitorHandle, Self::DeserializationError> {
        for handle in event_loop.available_monitors() {
            match self.try_eq(&handle) {
                None => return Err(Self::DeserializationError::Disconnected),
                Some(false) => continue,
                Some(true) => return Ok(handle),
            }
        }
        Err(Self::DeserializationError::NoMatch)
    }
}
