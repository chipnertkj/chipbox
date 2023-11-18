use crate::error;
use chipbox_common as common;
use std::fmt;
use std::path::PathBuf;
use tokio::fs;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(error::io::Error),
    Serde(error::serde::Error),
    HomeDir,
}

impl From<error::io::Error> for Error {
    fn from(e: error::io::Error) -> Self {
        Error::Io(e)
    }
}
impl From<error::serde::Error> for Error {
    fn from(e: error::serde::Error) -> Self {
        Error::Serde(e)
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Serde(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io error during settings read: {}", e),
            Error::Serde(e) => {
                write!(f, "serde error during settings read: {}", e)
            }
            _ => write!(f, "`HOME` env variable not set"),
        }
    }
}

pub trait SettingsExt<S> {
    fn file_path() -> Result<PathBuf>;
    async fn read() -> Result<S>;
    async fn write(&self) -> Result<()>;
}

impl SettingsExt<common::Settings> for common::Settings {
    fn file_path() -> Result<PathBuf> {
        const DATA_DIR: &str = ".chipbox";
        const FILENAME: &str = "settings.json";
        let home_path = home::home_dir().ok_or(Error::HomeDir)?;
        let path = home_path
            .join(DATA_DIR)
            .join(FILENAME);
        let canonical = path
            .canonicalize()
            .map_err(|e| error::io::Error { e, path })?;
        Ok(canonical)
    }

    async fn read() -> Result<Self> {
        let path = Self::file_path()?;
        let data = fs::read_to_string(&path)
            .await
            .map_err(|e| {
                let path = path.to_owned();
                error::io::Error { e, path }
            })?;
        let settings = serde_json::from_str(&data)
            .map_err(|e| error::serde::Error { e, path })?;
        Ok(settings)
    }

    async fn write(&self) -> Result<()> {
        let path = Self::file_path()?;
        let data = serde_json::to_string(&self).map_err(|e| {
            let path = path.to_owned();
            error::serde::Error { e, path }
        })?;
        fs::write(&path, data)
            .await
            .map_err(|e| error::io::Error { e, path })?;
        Ok(())
    }
}
