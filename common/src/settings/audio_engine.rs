use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Settings {
    pub host: SelectedHost,
    pub output_device: SelectedDevice,
    pub output_stream_config: StreamConfig,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub enum SelectedHost {
    #[default]
    Default,
    Named(String),
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub enum SelectedDevice {
    #[default]
    Default,
    Named(String),
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub enum StreamConfig {
    #[default]
    Default,
    Custom {
        sample_format: String,
        sample_rate: u32,
        channels: u16,
    },
}
