use async_std::fs;
use async_trait::async_trait;

#[async_trait]
pub trait TomlConfigTrait:
    serde::Serialize + serde::de::DeserializeOwned + Default + super::ConfigTrait
{
    /// The filename of the configuration file. It should include the file extension.
    fn config_file_name() -> &'static str;
    fn validate_and_fix(
        &mut self,
        validation_type: super::ValidationType,
    ) -> bool;
}

#[async_trait]
impl<T> super::ConfigTrait for T
where
    T: TomlConfigTrait + Send + Sync,
{
    fn config_file_name() -> &'static str {
        <T as TomlConfigTrait>::config_file_name()
    }

    fn validate_and_fix(
        &mut self,
        validation_type: super::ValidationType,
    ) -> bool {
        <T as TomlConfigTrait>::validate_and_fix(self, validation_type)
    }

    async fn save_raw(&mut self) -> anyhow::Result<()> {
        let config_folder = Self::config_folder();
        if !config_folder.exists() {
            fs::create_dir(config_folder).await?;
        }
        let toml = toml::to_string(self)?;
        fs::write(Self::config_file_path(), toml).await?;
        Ok(())
    }

    async fn load_raw() -> anyhow::Result<Self> {
        let toml = fs::read_to_string(Self::config_file_path()).await?;
        let settings = toml::from_str(toml.as_str())?;
        Ok(settings)
    }
}
