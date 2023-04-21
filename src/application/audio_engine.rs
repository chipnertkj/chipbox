use super::config::{self, ConfigTrait as _, StringSerializedTrait as _};

mod output;

pub struct AudioEngine {
    host_opt: Option<cpal::Host>,
    config: config::AudioEngineConfig,
}

impl AudioEngine {
    /// Returns the default `cpal::HostId` as defined by `cpal`.
    pub fn default_host_id() -> &'static cpal::HostId {
        // last host is the same as default in cpal impl
        cpal::ALL_HOSTS.last().expect("expected at least one audio backend to be availabe on this platform")
    }

    /// Constructs an `AudioEngine` according to the supplied config.
    pub fn with_config(mut config: config::AudioEngineConfig) -> Self {
        // attempt to deserialize host id
        let host_id_opt_serialized = config
            .host_id_opt_serialized
            .to_owned();
        let host_id_opt = host_id_opt_serialized
            .deserialize(())
            .unwrap_or_else(|e| {
                tracing::error!(
                    "Using default host due to invalid host config: {e}"
                );
                Some(*Self::default_host_id())
            });

        // update config in case host_id was changed
        config.host_id_opt_serialized = host_id_opt.into();

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

        Self { host_opt, config }
    }

    /// Returns the currently used host.
    pub fn host_opt(&self) -> &Option<cpal::Host> {
        &self.host_opt
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
        self.config
            .host_id_opt_serialized = host_id_opt.into();

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
        self.config.save_tracing()
    }
}
