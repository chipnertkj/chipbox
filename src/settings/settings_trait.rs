use std::{fs, path};

pub trait SettingsTrait:
    serde::Serialize + serde::de::DeserializeOwned
{
    fn config_file_name() -> &'static str;

    fn config_file_path() -> path::PathBuf {
        Self::config_folder().join(Self::config_file_name())
    }

    fn config_folder() -> path::PathBuf {
        home::home_dir()
            .expect("expected the HOME env var to be set")
            .join(".chipbox")
    }

    fn load() -> anyhow::Result<Self> {
        let toml = fs::read_to_string(Self::config_file_path())?;
        let settings = toml::from_str(toml.as_str())?;
        Ok(settings)
    }

    fn save(&self) -> anyhow::Result<()> {
        let config_folder = Self::config_folder();
        if !config_folder.exists() {
            fs::create_dir(config_folder)?;
        }
        let toml = toml::to_string(self)?;
        fs::write(Self::config_file_path(), toml)?;
        Ok(())
    }
}
