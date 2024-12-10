//! Defines an error type for filesystem operations.

use std::{io, path::PathBuf};

/// Error type for filesystem operations.
///
/// Holds information about an error related to a particular file or directory.
#[derive(thiserror::Error, Debug)]
pub enum FsError {
    /// Encountered an IO error.
    #[error("{path:?}: io error: {io}")]
    IO {
        /// The error that occurred.
        io: io::Error,
        /// The file or directory that was being accessed when the error occurred.
        path: PathBuf,
    },
    /// Attempted to access the parent directory of a path that does not have one.
    #[error("{path:?}: no parent directory")]
    NoParentDirectory {
        /// The file or directory that was being accessed when the error occurred.
        path: PathBuf,
    },
}

/// Convenience [`Result`] alias for [`FsError`].
pub type FsResult<T, E = FsError> = Result<T, E>;

/// Extension trait for [`std::io::Result`].
///
/// Used to convert IO errors into the [`FsError`] type.
pub trait FromIo<T> {
    /// Maps an IO error into the [`FsError`] type and includes
    /// the path that was being accessed when the error occurred.
    fn map_fs_err(self, path: impl Into<PathBuf>) -> FsResult<T>;
}

impl<T> FromIo<T> for io::Result<T> {
    fn map_fs_err(self, path: impl Into<PathBuf>) -> FsResult<T> {
        self.map_err(|e| FsError::IO {
            io: e,
            path: path.into(),
        })
    }
}
