//! File system utilities.

use std::path::{Path, PathBuf};

use get_dir::{FileTarget, GetDir, Target, tokio::GetDirAsyncExt as _};
use tokio::fs::{self};

use crate::command::CargoProfile;

#[derive(Debug, thiserror::Error)]
#[error("at: {path}")]
pub struct PathError {
    #[source]
    pub e: std::io::Error,
    pub path: PathBuf,
}

impl PathError {
    pub fn new(e: std::io::Error, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        Self { e, path }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FsError {
    /// Failed to get the current working directory.
    #[error("failed to get current directory")]
    GetCwd(#[source] std::io::Error),
    /// Failed to access a path.
    #[error("failed to access path")]
    AtPath(#[source] PathError),
    /// Failed to sanitize a path.
    #[error("failed to sanitize path")]
    Sanitize(#[source] PathError),
    /// Failed to create a directory.
    #[error("failed to create directory")]
    CreateDir(#[source] std::io::Error),
    /// Failed to open a file.
    #[error("failed to open file")]
    OpenFile(#[source] std::io::Error),
}

impl FsError {
    /// Shorthand constructor for [`FsError::AtPath`].
    pub fn at_path(e: std::io::Error, path: impl Into<PathBuf>) -> Self {
        Self::AtPath(PathError::new(e, path))
    }

    /// Shorthand constructor for [`FsError::Sanitize`].
    pub fn sanitize(e: std::io::Error, path: impl Into<PathBuf>) -> Self {
        Self::Sanitize(PathError::new(e, path))
    }
}

/// Sanitize a file path to ensure cross-platform compatibility.
pub async fn sanitize_path(dir: impl AsRef<Path>) -> Result<PathBuf, FsError> {
    let dir = dir.as_ref();
    let dir = fs::canonicalize(dir)
        .await
        .map_err(|e| FsError::sanitize(e, dir))?;
    Ok(dir)
}

/// Looks for any directory containing a file with the given name, in reverse order.
///
/// Starts from the provided directory and walks up the directory tree.
pub async fn ancestor_with_file(
    dir: impl AsRef<Path>,
    filename: impl AsRef<str>,
) -> Result<PathBuf, FsError> {
    let dir = sanitize_path(dir).await?;
    let target = Target::File(FileTarget::new(filename.as_ref()));
    GetDir {
        dir: dir.clone(),
        targets: vec![target],
        ..Default::default()
    }
    .run_reverse_async()
    .await
    .map_err(|e| FsError::at_path(e, dir))
}

/// Attempts to find the root directory of a cargo workspace.
/// Searches from the current working directory, based on the presence of a `Cargo.lock` file.
pub async fn cargo_workspace() -> Result<PathBuf, FsError> {
    let dir = std::env::current_dir().map_err(FsError::GetCwd)?;
    ancestor_with_file(&dir, "Cargo.lock").await
}

/// Attempts to find the root directory of a pnpm workspace.
/// Searches from the current working directory, based on the presence of a `pnpm-workspace.yaml` file.
pub async fn pnpm_workspace() -> Result<PathBuf, FsError> {
    let dir = std::env::current_dir().map_err(FsError::GetCwd)?;
    ancestor_with_file(&dir, "pnpm-workspace.yaml").await
}

/// Opens a file for appending and clears its contents beforehand.
pub async fn open_clear_file(path: impl AsRef<Path>) -> Result<fs::File, FsError> {
    let path = path.as_ref();
    // Create the directory.
    fs::create_dir_all(path.parent().expect("parent dir exists"))
        .await
        .map_err(FsError::CreateDir)?;
    // Create the file.
    fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)
        .await
        .map_err(FsError::OpenFile)?;
    // Open the file for appending.
    let out_file = fs::File::options()
        .append(true)
        .open(&path)
        .await
        .map_err(FsError::OpenFile)?;
    Ok(out_file)
}

/// Returns a path to the output directory for a given cargo profile.
pub async fn cargo_output_dir(profile: CargoProfile) -> Result<PathBuf, FsError> {
    let path = crate::fs::cargo_workspace()
        .await?
        .join(format!("target/{}", profile.target_folder(),));
    Ok(path)
}

/// The file to execute for [`Self::chipbox`].
pub fn chipbox_filename() -> &'static str {
    cfg!(windows).then_some("chipbox.exe").unwrap_or("chipbox")
}

/// Returns a path to the `chipbox` binary for a given cargo profile.
pub async fn built_chipbox_binary(profile: CargoProfile) -> Result<PathBuf, FsError> {
    let path = crate::fs::cargo_workspace().await?.join(format!(
        "target/{}/{}",
        profile.target_folder(),
        chipbox_filename()
    ));
    Ok(path)
}
