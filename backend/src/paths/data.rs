use chipbox_common::error;
use error::fs::FromIo as _;
use std::path::PathBuf;
use tokio::fs;

/// Error type for accessing the application data directory.
#[derive(Debug, thiserror::Error)]
pub(crate) enum DataDirError {
    #[error("file error: {0}")]
    Fs(#[from] error::fs::FsError),
    #[error("unable to determine home directory")]
    HomeDir,
}

/// Convenience [`Result`](std::result::Result) alias for [`DataDirError`].
pub(crate) type DataDirResult<T, E = DataDirError> = std::result::Result<T, E>;

/// Constructs the path to a file in the application data directory.
///
/// The directory is created if it does not exist.
/// The returned path is canonicalized.
pub(crate) async fn dir_path() -> DataDirResult<PathBuf> {
    // Retrieve the path to the HOME directory.
    let home_path = home::home_dir().ok_or(DataDirError::HomeDir)?;
    // Construct the path.
    let dir_path = home_path.join(".chipbox");
    // Ensure the directory exists.
    fs::create_dir_all(&dir_path)
        .await
        .map_fs_err(dir_path.clone())?;
    // Convert the path to a canonical path.
    let canonical = dir_path.canonicalize().map_fs_err(dir_path.clone())?;
    Ok(canonical)
}
