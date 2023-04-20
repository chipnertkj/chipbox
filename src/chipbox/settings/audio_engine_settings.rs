mod host_id_serialized;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AudioEngineSettings {
    pub host_id_serialized: host_id_serialized::HostIdSerialized,
}

impl Default for AudioEngineSettings {
    fn default() -> Self {
        Self {
            host_id_serialized: cpal::default_host()
                .id()
                .into(),
        }
    }
}

impl super::SettingsTrait for AudioEngineSettings {
    fn config_file_name() -> &'static str {
        "audio_engine_settings.toml"
    }
}
