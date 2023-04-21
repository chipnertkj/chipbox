use crate::application::audio_engine;

mod host_id_opt_serialized;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AudioEngineConfig {
    pub host_id_opt_serialized: host_id_opt_serialized::HostIdOptSerialized,
}

impl Default for AudioEngineConfig {
    fn default() -> Self {
        Self {
            host_id_opt_serialized: Some(
                *audio_engine::AudioEngine::default_host_id(),
            )
            .into(),
        }
    }
}

impl super::ConfigTrait for AudioEngineConfig {
    fn config_file_name() -> &'static str {
        "audio_engine_config.toml"
    }
}
