//! Contains hard-coded paths used throughout the application,
//! as well as filesystem utilities for unit tests.

use std::path::PathBuf;
use std::{error, fmt, io};

/// Result type alias for this module's `Error` type.
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Errors encountered while accessing paths defined in this module.
#[derive(Debug)]
pub(crate) enum Error {
    /// See inner type for more information.
    IO(io::Error),
    /// Unable to locate the user's HOME directory.
    HomeDirectory,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(e) => {
                write!(f, "io error while reading path: {e}`")
            }
            Self::HomeDirectory => {
                f.write_str("unable to locate the user's HOME directory")
            }
        }
    }
}

impl error::Error for Error {}

/// Path to the app data folder.
/// Local to the user's `HOME` directory.
const MAIN_FOLDER: &str = "./.chipbox/";

/// Path to the config folder.
/// Local to the user's `HOME` directory.
const CONFIG_FOLDER: &str =
    const_format::formatcp!("{main}./config/", main = MAIN_FOLDER);

/// Default path to the projects folder.
/// Local to the `MAIN_FOLDER` directory.
pub(crate) const DEFAULT_PROJECTS_FOLDER: &str = "./projects/";

/// Absoulte path to the main application data folder.
///
/// # Notes
/// See `create_temp_dir` for more information about the behavior
/// of this function in unit tests.
pub(crate) fn main_folder() -> Result<PathBuf> {
    #[cfg(not(test))]
    {
        let path = home::home_dir()
            .ok_or(Error::HomeDirectory)?
            .join(MAIN_FOLDER)
            .canonicalize()
            .map_err(Error::IO)?;
        Ok(path)
    }
    #[cfg(test)]
    {
        let path = temp_path()
            .canonicalize()
            .map_err(Error::IO)?;
        Ok(path)
    }
}

/// Absoulte path to the config folder.
pub(crate) fn config_folder() -> Result<PathBuf> {
    let path = main_folder()?
        .join(CONFIG_FOLDER)
        .canonicalize()
        .map_err(Error::IO)?;
    Ok(path)
}

#[cfg(test)]
use std::sync::Arc;
#[cfg(test)]
use std::sync::{OnceLock, Weak};
#[cfg(test)]
use tempfile::TempDir;

#[cfg(test)]
/// Shared state between the utility functions `create_temp_dir` and `temp_path`.
/// See their documentation for more information.
static TEMP_DIR_WEAK: OnceLock<Weak<TempDir>> = OnceLock::new();

/// Utility function for filesystem-dependent unit tests.
/// Returns an `Arc` to a handle owning a temporary directory.
///
/// A weak pointer to the directory is stored statically after call.
/// It is used to substitute the HOME directory in functions like `main_folder` and `config_folder`.
///
/// More precisely, the `HOME/{MAIN_FOLDER}` directory is replaced with the temp folder.
///
/// The folder will be deleted as soon as the inner value is dropped. See `std::sync::Arc`.
///
/// # Notes
///
/// If `create_temp_dir` is not called before calling any of the path functions that depend on `create_temp_dir`, they will panic.
/// See `crate::path::temp_path` for implementation details.
///
/// This function should be called ***at least once*** at the start of each unit test that takes advantage of filesystem operations.
/// Otherwise, the directory will not be created.
/// The `Arc` should be kept in scope for as long as filesystem operations are expected to be used.
#[cfg(test)]
pub(crate) fn create_temp_dir() -> std::result::Result<Arc<TempDir>, io::Error>
{
    let init = || {
        let temp_dir = Arc::new(tempfile::tempdir()?);
        TEMP_DIR_WEAK.get_or_init(|| std::sync::Arc::downgrade(&temp_dir));
        Ok(temp_dir)
    };
    match TEMP_DIR_WEAK.get() {
        Some(weak) => match weak.upgrade() {
            Some(temp_dir) => Ok(temp_dir),
            None => init(),
        },
        None => init(),
    }
}

/// Attempt to retrieve a filesystem path of the temporary directory.
/// See `create_temp_dir` for more information.
#[cfg(test)]
fn temp_path() -> PathBuf {
    TEMP_DIR_WEAK
        .get()
        .expect("temp dir was not initialized. see `crate::path::create_temp_dir` for more info")
        .upgrade()
        .expect("temp dir was dropped. see `crate::path::create_temp_dir` for more info")
        .path()
        .to_path_buf()
}
