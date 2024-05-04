use std::path::{Path, PathBuf};

use crate::error::io::IoError;
use tokio::fs;

/// Name of application data directory.
const DATA_DIR: &str = ".chipbox";

#[derive(Debug)]
/// Describes an error that occurred while reading the settings file.
pub enum DataDirError {
    /// An I/O error occurred.
    Io(IoError),
    /// The `HOME` environment variable was not set.
    HomeDir,
}

/// Convenience conversion.
impl From<IoError> for DataDirError {
    fn from(err: IoError) -> Self {
        DataDirError::Io(err)
    }
}

/// Error implementation.
impl std::error::Error for DataDirError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DataDirError::Io(err) => Some(err),
            DataDirError::HomeDir => None,
        }
    }
}

/// Display implementation.
impl std::fmt::Display for DataDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataDirError::Io(err) => {
                write!(f, "io error during settings read: {}", err)
            }
            DataDirError::HomeDir => {
                write!(f, "`HOME` env variable not set")
            }
        }
    }
}

/// Constructs the path to a file in the application data directory.
///
/// Asserts the existence of its parent directory with `fs::create_dir_all`.
/// # Errors
/// List of possible errors:
/// - An I/O error occurred.
/// - The `HOME` environment variable was not set.
pub async fn data_path() -> Result<std::path::PathBuf, DataDirError> {
    // Retrieve the path to the HOME directory.
    let home_path = home::home_dir().ok_or(DataDirError::HomeDir)?;

    // Construct the path.
    let path = home_path.join(DATA_DIR);

    // Ensure the directory exists.
    fs::create_dir_all(&path)
        .await
        .map_err(|err| IoError {
            err,
            path: path.clone(),
        })?;

    // Convert the path to a canonical path.
    let canonical = path
        .canonicalize()
        .map_err(|err| IoError { err, path })?;

    // Return final data path.
    Ok(canonical)
}
