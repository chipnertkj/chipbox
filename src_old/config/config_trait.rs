use async_trait::async_trait;
use std::path;

pub mod toml;

pub enum ValidationType {
    Save,
    Load,
}

#[async_trait]
pub trait ConfigTrait:
    serde::Serialize + serde::de::DeserializeOwned + Default
{
    /// The filename of the configuration file. It should include the file extension.
    fn config_file_name() -> &'static str;

    /// The path of the configuration file, including the filename.
    fn config_file_path() -> path::PathBuf {
        Self::config_folder().join(Self::config_file_name())
    }

    /// The location of the configuration file.
    fn config_folder() -> path::PathBuf {
        home::home_dir()
            .expect("expected the HOME env var to be set")
            .join(".chipbox")
    }

    /// Checks for errors in the config on save and load. Applies fixes, so that the config is valid afterwards.
    fn validate_and_fix(&mut self, validation_type: ValidationType) -> bool;

    /// Deserializes Self without validation or tracing.
    async fn load_raw() -> anyhow::Result<Self>;

    /// Deserializes Self with validation, provides validation tracing.
    async fn load() -> anyhow::Result<Self> {
        let mut config = Self::load_raw().await?;
        tracing::info!("Validating config '{}'...", Self::config_file_name());
        if config.validate_and_fix(ValidationType::Load) {
            tracing::info!(
                "Load validation ok for '{}'.",
                Self::config_file_name()
            );
        } else {
            tracing::error!(
                "Load validation failed for '{}'. Applied fix.",
                Self::config_file_name()
            );
        }
        Ok(config)
    }

    /// Deserializes Self with validation. Unwraps to Some(_) or Default::default() and provides error tracing.
    async fn load_or_default_tracing() -> Self {
        match Self::load().await {
            Ok(v) => {
                tracing::info!("Loaded config '{}'.", Self::config_file_name());
                v
            }
            Err(e) => {
                tracing::warn!(
                    "Unable to load config '{}', using default config: {e}",
                    Self::config_file_name()
                );
                Default::default()
            }
        }
    }

    /// Serializes Self without validation or tracing.
    async fn save_raw(&mut self) -> anyhow::Result<()>;

    /// Serializes Self with validation, provides validation tracing.
    async fn save(&mut self) -> anyhow::Result<()> {
        if self.validate_and_fix(ValidationType::Save) {
            tracing::info!(
                "Save validation ok for '{}'.",
                Self::config_file_name()
            );
        } else {
            tracing::error!(
                "Save validation failed for '{}'. Applied fix.",
                Self::config_file_name()
            );
        }
        self.save_raw().await
    }

    /// Saves the config with `Self::save` and provides error tracing.
    async fn save_tracing(&mut self) {
        match self.save().await {
            Ok(_) => {
                tracing::info!("Saved config '{}'.", Self::config_file_name())
            }
            Err(e) => {
                tracing::error!(
                    "Unable to save config '{}': {e}",
                    Self::config_file_name()
                )
            }
        }
    }
}
