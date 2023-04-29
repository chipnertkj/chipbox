use crate::config;
use config::StringSerializedTrait as _;

mod host_id_serialized;
pub use host_id_serialized::{HostIdDeserializationError, HostIdSerialized};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AudioEngineConfig {
    pub host_id_serialized: HostIdSerialized,
}

impl AudioEngineConfig {
    /// Returns the default `cpal::HostId` as defined by `cpal`.
    pub fn default_host_id() -> &'static cpal::HostId {
        // Last host should be the same as default.
        cpal::ALL_HOSTS.last().expect("expected at least one audio backend to be availabe on this platform")
    }
}

impl Default for AudioEngineConfig {
    fn default() -> Self {
        let host_id = *Self::default_host_id();
        let host_id_serialized = HostIdSerialized::serialize(host_id);
        Self { host_id_serialized }
    }
}

impl config::ConfigTrait for AudioEngineConfig {
    fn config_file_name() -> &'static str {
        "audio_engine_config.toml"
    }
}

#[cfg(test)]
mod test {
    #[test]
    /// Assert that `AudioEngineConfig::default_host_id()` matches the actual default host used by `cpal`.
    fn default_host_id() {
        let id_from_default_host = cpal::default_host().id();
        assert_eq!(
            *super::AudioEngineConfig::default_host_id(),
            id_from_default_host
        );
    }
}
