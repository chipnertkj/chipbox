mod setting;

pub use setting::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {}

impl Settings {
    pub fn file_path(home_directory: &std::path::Path) -> std::path::PathBuf {
        home_directory.join(r".chipbox/user_config/settings.json")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SettingsLoadError {
    /// This should be an `std::io::Error`, but we use a `String` for serialization purposes.
    IOError {
        inner: String,
    },
    /// This should be an `serde_json::Error`, but we use a `String` serialization for purposes.
    DeserializationError {
        inner: String,
    },
    NoHomeDirectory {
        inner: String,
    },
    NotFound,
}

impl std::fmt::Display for SettingsLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError { inner } => {
                write!(f, "cannot load settings due to I/O error: {inner}")
            }
            Self::DeserializationError { inner } => write!(
                f,
                "cannot load settings due to deserialization error: {inner}"
            ),
            Self::NoHomeDirectory { inner } => {
                write!(f, "cannot load settings from home directory: {inner}")
            }
            Self::NotFound => f.write_str("no settings file found"),
        }
    }
}

impl std::error::Error for SettingsLoadError {}
