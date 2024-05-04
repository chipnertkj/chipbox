use crate::error;
use chipbox_common as common;
use std::path::PathBuf;
use tokio::fs;

/// Result type alias for module error type.
pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
/// Describes an error that occurred while reading the settings file.
pub enum Error {
    /// An I/O error occurred.
    Io(error::io::Error),
    /// A serialization/deserialization error occurred.
    Serde(error::serde::Error),
    /// The `HOME` environment variable was not set.
    HomeDir,
}

/// Convenience conversion.
impl From<error::io::Error> for Error {
    fn from(e: error::io::Error) -> Self {
        Error::Io(e)
    }
}

/// Convenience conversion.
impl From<error::serde::Error> for Error {
    fn from(e: error::serde::Error) -> Self {
        Error::Serde(e)
    }
}

/// Error implementation.
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Serde(e) => Some(e),
            _ => None,
        }
    }
}

/// Display implementation.
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io error during settings read: {}", e),
            Error::Serde(e) => {
                write!(f, "serde error during settings read: {}", e)
            }
            _ => write!(f, "`HOME` env variable not set"),
        }
    }
}

/// Extension trait for `Settings`.
///
/// This trait provides methods for reading and writing a settings file.
pub trait SettingsExt<S> {
    /// Returns the path to the settings file.
    /// Ensures the directory exists.
    async fn file_path() -> Result<PathBuf>;
    /// Reads the settings file.
    async fn read() -> Result<S>;
    /// Writes the settings file.
    async fn write(&self) -> Result<()>;
}

/// Extends `common::Settings` with methods for reading and writing a
/// settings file.
impl SettingsExt<common::Settings> for common::Settings {
    async fn file_path() -> Result<PathBuf> {
        /// Name of the directory where settings are stored.
        const DATA_DIR: &str = ".chipbox";
        /// Name of the settings file.
        const FILENAME: &str = "settings.json";

        // Retrieve the path to the HOME directory.
        let home_path = home::home_dir().ok_or(Error::HomeDir)?;
        // Construct the path to the settings file.
        let path = home_path
            .join(DATA_DIR)
            .join(FILENAME);

        // Ensure the directory exists.
        fs::create_dir_all(path.parent().unwrap())
            .await
            .map_err(|e| error::io::Error {
                inner: e,
                path: path.clone(),
            })?;

        // Convert the path to a canonical path.
        let canonical = path
            .canonicalize()
            .map_err(|e| error::io::Error { inner: e, path })?;
        // Return path to settings file.
        Ok(canonical)
    }

    async fn read() -> Result<Self> {
        // Retrieve the path to the settings file.
        let path = Self::file_path().await?;

        // Read the settings file.
        let data = fs::read_to_string(&path)
            .await
            .map_err(|e| {
                let path = path.to_owned();
                error::io::Error { inner: e, path }
            })?;

        // Parse the settings file.
        let settings = serde_json::from_str(&data)
            .map_err(|e| error::serde::Error { e, path })?;

        // Return settings.
        Ok(settings)
    }

    async fn write(&self) -> Result<()> {
        // Retrieve the path to the settings file.
        let path = Self::file_path().await?;

        // Serialize the settings file.
        let data = serde_json::to_string(&self).map_err(|e| {
            let path = path.to_owned();
            error::serde::Error { e, path }
        })?;

        // Write the settings file.
        fs::write(&path, data)
            .await
            .map_err(|e| error::io::Error { inner: e, path })?;

        // All done.
        Ok(())
    }
}
