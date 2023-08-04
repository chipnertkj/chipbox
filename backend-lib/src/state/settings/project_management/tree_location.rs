//! Defines the `ProjectTreeLocation` setting, responsible for
//! storing the location of a user's project `Tree` in the filesystem.

use crate::path;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{error, fmt, io};

/// Result type alias for this module's `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors encountered while generating a filesystem path from the `ProjectTreeLocation` setting.
#[derive(Debug)]
pub enum Error {
    /// Selected `ProjectTreeLocation` variant is `MainFolderRelative`,
    /// but the inner `PathBuf` is absolute.
    NotRelative(PathBuf),
    /// Selected `ProjectTreeLocation` variant is `Absolute`,
    /// but the inner `PathBuf` is relative.
    NotAbsolute(PathBuf),
    /// See inner type for more information.
    Path(path::Error),
    /// See inner type for more information.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRelative(path) => {
                write!(
                    f,
                    "`project tree location` setting is set to `main folder relative`, but the `main folder relative` path is absolute: `{path:?}`"
                )
            }
            Self::NotAbsolute(path) => {
                write!(
                    f,
                    "`project tree location` setting is set to `absolute`, but the `absolute` path is relative: `{path:?}`"
                )
            }
            Self::Path(e) => write!(
                f,
                "encountered a `path::Error` during `project tree location` path generation: {e}"
            ),
            Self::Io(e) => write!(
                f,
                "encountered an `io::Error` during `project tree location` path generation: {e}"
            ),
        }
    }
}

impl error::Error for Error {}

/// Location of a user's project `Tree` in the filesystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreeLocation {
    /// The inner `PathBuf` should be an absolute path.
    ///
    /// The value returned by `Self::path` will be the inner `PathBuf`.
    Absolute(PathBuf),
    /// The inner `PathBuf` should be a relative path.
    ///
    /// The value returned by `Self::path` will be the inner `PathBuf`,
    /// concatenated with the absolute path of the main folder.
    MainFolderRelative(PathBuf),
}

impl TreeLocation {
    /// Attempt to convert the virtual representation of an absolute path
    /// to an absolute, canonical filesystem path.
    fn absolute_to_path_buf(inner: &Path) -> Result<PathBuf> {
        if inner.is_absolute() {
            let path = inner
                .canonicalize()
                .map_err(Error::Io)?;
            Ok(path)
        } else {
            Err(Error::NotAbsolute(inner.to_owned()))
        }
    }

    /// Attempt to convert the virtual representation of a main-folder-relative path
    /// to an absolute, canonical filesystem path.
    fn relative_to_path_buf(inner: &Path) -> Result<PathBuf> {
        if inner.is_relative() {
            let path = path::main_folder()
                .map_err(Error::Path)?
                .join(inner)
                .canonicalize()
                .map_err(Error::Io)?;
            Ok(path)
        } else {
            Err(Error::NotRelative(inner.to_owned()))
        }
    }

    /// Construct an absolute, canonical filesystem path based on the variant of `self`.
    pub fn path(&self) -> Result<PathBuf> {
        match self {
            Self::Absolute(p) => Self::absolute_to_path_buf(p),
            Self::MainFolderRelative(p) => Self::relative_to_path_buf(p),
        }
    }
}

impl Default for TreeLocation {
    /// Constructs a `Self::MainFolderRelative` using `path::DEFAULT_PROJECTS_FOLDER`.
    fn default() -> Self {
        Self::MainFolderRelative(
            PathBuf::from_str(path::DEFAULT_PROJECTS_FOLDER)
                .expect("unreachable: expected '!' (never)"),
        )
    }
}
