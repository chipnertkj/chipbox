use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub struct Settings {
    pub host: SelectedHost,
    pub output_device: SelectedDevice,
    pub output_stream_config: StreamConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub enum SelectedHost {
    #[default]
    Default,
    Named(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub enum SelectedDevice {
    #[default]
    Default,
    Named(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub enum StreamConfig {
    #[default]
    Default,
    Custom {
        sample_format: String,
        sample_rate: u32,
        buffer_duration: Duration,
        channels: u16,
    },
}

impl StreamConfig {
    pub fn buffer_duration(&self) -> Duration {
        match self {
            StreamConfig::Default => Duration::from_millis(150),
            StreamConfig::Custom {
                buffer_duration, ..
            } => *buffer_duration,
        }
    }
}
