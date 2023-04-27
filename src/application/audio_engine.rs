use crate::config::{ConfigTrait as _, StringSerializedTrait as _};

mod config;
mod output;

/// This struct represents the current state of the audio pipeline.
/// It also presents an interface for managing resources such as devices and streams.
///
/// It also uses a configuration file (see `AudioEngineConfig`) to retain user setup between sessions.
/// An `AudioEngine` in `Disabled` state does not have any devices or streams, but it
/// still retains the desired setup as a loaded config.
///
/// When changing the underlying `cpal::Host`, it will attempt to open the same devices/streams based on the config.
#[allow(unused)]
pub enum AudioEngine {
    Disabled,
    Enabled {
        config: config::AudioEngineConfig,
        host: cpal::Host,
    },
}

#[allow(unused)]
impl AudioEngine {
    /// Returns a disabled `AudioEngine` with a loaded config.
    pub fn disabled() -> Self {
        Self::Disabled
    }

    /// Loads the `AudioEngineConfig` and constructs an enabled `AudioEngine` based on it.
    /// Overrides the audio backend to `cpal::HostId`.
    pub fn enabled_with_host(host_id: cpal::HostId) -> Self {
        let mut config = config::AudioEngineConfig::load_or_default_tracing();
        // Modify config before passing it to the constructor.
        config.host_id_serialized =
            config::HostIdSerialized::serialize(host_id);
        Self::enabled_with_config(config)
    }

    /// Constructs an enabled `AudioEngine` based on an `AudioEngineConfig`.
    fn enabled_with_config(mut config: config::AudioEngineConfig) -> Self {
        let host_id_result = config
            .host_id_serialized
            .deserialize(());

        match host_id_result {
            Ok(host_id) => {
                let host = cpal::host_from_id(host_id)
                    .expect("expected host_id to be valid");
                tracing::info!("Selected host: {}", host_id.name());
                Self::Enabled { host, config }
            }
            Err(e) => {
                // Revert to default host in case host_id deserialization fails.
                tracing::error!("Unable to select host: {e}");
                tracing::warn!("Reverting to default host.");
                let host_id = *config::AudioEngineConfig::default_host_id();
                // Remember to update config.
                config.host_id_serialized =
                    config::HostIdSerialized::serialize(host_id);
                // It's ok to call this recursively here, as `config.host_id_serialized` should be valid.
                Self::enabled_with_config(config)
            }
        }
    }

    /// Loads the confiig and constructs an enabled `AudioEngine` based on it.
    pub fn enabled_load_config() -> Self {
        let config = config::AudioEngineConfig::load_or_default_tracing();
        Self::enabled_with_config(config)
    }
}

impl Drop for AudioEngine {
    /// Save config and log on drop.
    fn drop(&mut self) {
        if let Self::Enabled { config, .. } = self {
            config.save_tracing()
        }
    }
}
