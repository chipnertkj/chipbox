use crate::dir::{self, DataDirError};
use crate::error::io::IoError;
use crate::error::serde::SerdeError;
use chipbox_common as common;
use common::Settings;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug)]
/// Describes an error that occurred while reading the settings file.
pub enum SettingsError {
    /// An I/O error occurred.
    Io(IoError),
    /// A serialization/deserialization error occurred.
    Serde(SerdeError),
    /// A data-dir operation failed. See `dir` module.
    DataDir(DataDirError),
}

/// Convenience conversion.
impl From<IoError> for SettingsError {
    fn from(err: IoError) -> Self {
        SettingsError::Io(err)
    }
}

/// Convenience conversion.
impl From<SerdeError> for SettingsError {
    fn from(err: SerdeError) -> Self {
        SettingsError::Serde(err)
    }
}

/// Convenience conversion.
impl From<DataDirError> for SettingsError {
    fn from(err: DataDirError) -> Self {
        SettingsError::DataDir(err)
    }
}

/// Error implementation.
impl std::error::Error for SettingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SettingsError::Io(err) => Some(err),
            SettingsError::Serde(err) => Some(err),
            SettingsError::DataDir(err) => Some(err),
        }
    }
}

/// Display implementation.
impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::Io(err) => {
                write!(f, "io error during settings read: {}", err)
            }
            SettingsError::Serde(err) => {
                write!(f, "serde error during settings read: {}", err)
            }
            SettingsError::DataDir(err) => {
                write!(f, "data-dir error during settings read: {}", err)
            }
        }
    }
}

/// Extension trait for `Settings`.
///
/// This trait provides methods for reading and writing a settings file.
pub trait SettingsExt<S> {
    /// Returns the path to the settings file.
    /// Ensures the directory exists.
    async fn file_path() -> Result<PathBuf, DataDirError>;

    /// Reads the settings file.
    async fn read() -> Result<S, SettingsError>;

    /// Writes the settings file.
    async fn write(&self) -> Result<(), SettingsError>;
}

/// Extends `common::Settings` with methods for reading and writing a
/// settings file.
impl SettingsExt<Settings> for Settings {
    /// Constructs the path to the settings file.
    async fn file_path() -> Result<PathBuf, DataDirError> {
        const FILENAME: &str = "settings.json";
        let path = dir::data_path()
            .await?
            .join(FILENAME);
        Ok(path)
    }

    /// Reads the settings file.
    async fn read() -> Result<Self, SettingsError> {
        // Retrieve the path to the settings file.
        let path = Self::file_path().await?;

        // Read the settings file.
        let data = fs::read_to_string(&path)
            .await
            .map_err(|err| {
                let path = path.to_owned();
                IoError { err, path }
            })?;

        // Parse the settings file.
        let settings = serde_json::from_str(&data)
            .map_err(|err| SerdeError { err, path })?;

        // Return settings.
        Ok(settings)
    }

    /// Writes the settings file.
    async fn write(&self) -> Result<(), SettingsError> {
        // Retrieve the path to the settings file.
        let path = Self::file_path().await?;

        // Serialize the settings file.
        let data = serde_json::to_string(&self).map_err(|err| {
            let path = path.to_owned();
            SerdeError { err, path }
        })?;

        // Write the settings file.
        fs::write(&path, data)
            .await
            .map_err(|err| IoError { err, path })?;

        // All done.
        Ok(())
    }
}
