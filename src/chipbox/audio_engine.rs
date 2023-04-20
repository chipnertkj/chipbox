use super::settings::{self, SettingsTrait};

pub struct AudioEngine {
    host_opt: Option<cpal::Host>,
    settings: settings::AudioEngineSettings,
}

impl AudioEngine {
    /// Constructs an `AudioEngine` according to the supplied config.
    pub fn new(mut settings: settings::AudioEngineSettings) -> Self {
        // attempt to deserialize host id
        let host_id_serialized = settings
            .host_id_serialized
            .to_owned();
        let host_id_opt = host_id_serialized
            .deserialize()
            .unwrap_or_else(|e| {
                tracing::error!(
                    "Using default host due to invalid host config: {e}"
                );
                Some(cpal::default_host().id())
            });

        // update config in case host_id was changed
        settings.host_id_serialized = host_id_opt.into();

        // initialize host or leave as None based on config
        let host_opt = host_id_opt
            .map(|x| cpal::host_from_id(x).expect("host is unavailable"));

        // log
        let host_name_opt = host_opt
            .as_ref()
            .map(|x| x.id().name());
        match host_name_opt {
            Some(host_name) => tracing::info!("Selected host: {host_name}"),
            None => tracing::info!(
                "Did not select a host (as per audio engine config)"
            ),
        };

        Self { host_opt, settings }
    }

    /// Changes the underlying `cpal::Host` and reaccesses audio resources.
    pub fn set_host(&mut self, host_id_opt: Option<cpal::HostId>) {
        let self_host_id_opt = self
            .host_opt
            .as_ref()
            .map(|x| x.id());

        // reaccess audio resources on host change
        let cleanup = || todo!("add host change cleanup code");
        let is_different = self_host_id_opt != host_id_opt;
        if is_different {
            cleanup();
        }

        // change host and update config
        self.host_opt = host_id_opt.map(|host_id| {
            cpal::host_from_id(host_id).expect("host is unavailable")
        });
        self.settings
            .host_id_serialized = host_id_opt.into();

        // log
        let self_host_name_opt = self
            .host_opt
            .as_ref()
            .map(|x| x.id().name());
        match self_host_name_opt {
            Some(host_name) => tracing::info!("Selected host: {host_name}"),
            None => tracing::info!("Selected host: None"),
        };
    }
}

impl Drop for AudioEngine {
    /// Save config and log on drop.
    fn drop(&mut self) {
        self.settings.save_tracing()
    }
}