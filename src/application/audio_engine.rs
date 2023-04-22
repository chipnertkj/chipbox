use super::config::{ConfigTrait as _, StringSerializedTrait as _};

mod audio_engine_config;
mod output;

pub enum AudioEngine {
    Disabled {
        config: audio_engine_config::AudioEngineConfig,
    },
    Enabled {
        host: cpal::Host,
        config: audio_engine_config::AudioEngineConfig,
    },
}

impl AudioEngine {
    /// Returns a disabled `AudioEngine` with a loaded config.
    pub fn disabled() -> Self {
        let config =
            audio_engine_config::AudioEngineConfig::load_or_default_tracing();
        Self::Disabled { config }
    }

    /// Loads the `AudioEngineConfig` and constructs an enabled `AudioEngine` based on it.
    /// Overrides the audio backend to `cpal::HostId`.
    pub fn with_host(host_id: cpal::HostId) -> Self {
        let mut config =
            audio_engine_config::AudioEngineConfig::load_or_default_tracing();
        config.host_id_serialized =
            audio_engine_config::HostIdSerialized::serialize(host_id);
        Self::with_config(config)
    }

    /// Constructs an enabled `AudioEngine` based on an `AudioEngineConfig`.
    fn with_config(mut config: audio_engine_config::AudioEngineConfig) -> Self {
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
                tracing::error!("Unable to select host: {e}");
                tracing::warn!("Reverting to default host.");
                let host_id =
                    *audio_engine_config::AudioEngineConfig::default_host_id();
                config.host_id_serialized =
                    audio_engine_config::HostIdSerialized::serialize(host_id);
                // It's ok to call this recursively, as `config.host_id_serialized` should be valid.
                Self::with_config(config)
            }
        }
    }

    /// Loads the `AudioEngineConfig` and constructs an enabled `AudioEngine` based on it.
    pub fn load_from_config() -> Self {
        let config =
            audio_engine_config::AudioEngineConfig::load_or_default_tracing();
        Self::with_config(config)
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
