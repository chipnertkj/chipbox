//! Defines `settings.json` and its deserialized counterpart, `Settings`,
//! as well as some underlying deserializable setting types.

pub mod project_management;
pub use self::project_management::{ProjectManagement, TreeLocation};

use crate::path;
use serde::{Deserialize, Serialize};
use std::{error, fmt, io};

/// Result type alias for this module's `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors encountered during `Settings` loading.
#[derive(Debug)]
pub enum Error {
    /// See inner type for more information.
    IO(io::Error),
    /// See inner type for more information.
    Path(path::Error),
    /// See inner type for more information.
    Serde(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(e) => {
                write!(f, "unable to load settings due to I/O error: {e}")
            }
            Self::Path(e) => {
                write!(f, "unable to load settings due to path error: {e}")
            }
            Self::Serde(e) => write!(
                f,
                "unable to load settings due to deserialization error: {e}"
            ),
        }
    }
}

impl error::Error for Error {}

/// Serde-compatible representation of the settings file.
/// Contains all user configuration.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub project_management: ProjectManagement,
}

#[cfg(feature = "backend")]
use tokio::fs;
#[cfg(feature = "backend")]
impl Settings {
    /// Reads the `settings.json` file in the `path::config_folder`
    /// and outputs the resulting `Settings`.
    pub async fn read() -> Result<Self> {
        const FILE_NAME: &str = "settings.json";
        let path = path::config_folder()
            .map_err(Error::Path)?
            .join(FILE_NAME);

        let data = fs::read_to_string(&path)
            .await
            .map_err(Error::IO)?;
        let settings =
            serde_json::from_str::<Settings>(&data).map_err(Error::Serde)?;

        Ok(settings)
    }
}
