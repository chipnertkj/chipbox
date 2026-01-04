//! File system utilities.

use std::path::{Path, PathBuf};

use get_dir::{DirTarget, FileTarget, GetDir, Target, tokio::GetDirAsyncExt as _};
use tokio::fs::{self};

use crate::CargoProfile;

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
    let dir = dunce::simplified(&dir).to_owned();
    Ok(dir)
}

async fn ancestor_with_target(dir: impl AsRef<Path>, target: Target) -> Result<PathBuf, FsError> {
    let dir = sanitize_path(dir).await?;
    GetDir {
        dir: dir.clone(),
        targets: vec![target],
        ..Default::default()
    }
    .run_reverse_async()
    .await
    .map_err(|e| FsError::at_path(e, dir))
}

/// Looks for any directory containing a file with the given name, in reverse order.
///
/// Starts from the provided directory and walks up the directory tree.
pub async fn ancestor_with_file(
    dir: impl AsRef<Path>,
    filename: impl AsRef<str>,
) -> Result<PathBuf, FsError> {
    let target = Target::File(FileTarget::new(filename.as_ref()));
    ancestor_with_target(dir, target).await
}

pub async fn ancestor_with_subdir(
    dir: impl AsRef<Path>,
    subdir: impl AsRef<str>,
) -> Result<PathBuf, FsError> {
    let target = Target::Dir(DirTarget::new(subdir.as_ref()));
    ancestor_with_target(dir, target).await
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

pub async fn chipbox_solid_render() -> Result<PathBuf, FsError> {
    Ok(pnpm_workspace().await?.join("node/chipbox-solid-render"))
}

pub async fn ts_bindings_output() -> Result<PathBuf, FsError> {
    Ok(chipbox_solid_render().await?.join("generated"))
}

pub async fn portaudio() -> Result<PathBuf, FsError> {
    Ok(cargo_workspace().await?.join("portaudio"))
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

/// Returns a path to a subdirectory in the cargo output directory.
pub async fn cargo_target_subdir(subdir: impl std::fmt::Display) -> Result<PathBuf, FsError> {
    let path = crate::fs::cargo_workspace()
        .await?
        .join(format!("target/{subdir}"));
    Ok(path)
}

/// Returns a path to the output directory for a given cargo profile.
pub async fn cargo_output_dir(profile: CargoProfile) -> Result<PathBuf, FsError> {
    let path = crate::fs::cargo_target_subdir(profile.target_folder()).await?;
    Ok(path)
}

/// The file to execute for [`Self::chipbox`].
#[must_use]
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
