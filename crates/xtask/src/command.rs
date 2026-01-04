//! Abtraction over [`std::process::Command`].

use std::{borrow::Cow, ffi::OsStr, path::Path};

pub use self::child::Child;
use crate::program::{ProgramAction, ProgramCommand, ProgramOutputId};

mod child;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    Fs(#[from] crate::fs::FsError),
    /// Failed to get the cargo workspace root directory.
    #[error("failed to get cargo workspace root")]
    CargoWorkspaceRoot(#[source] crate::fs::FsError),
    /// Failed to get the pnpm workspace root directory.
    #[error("failed to get pnpm workspace root")]
    PnpmWorkspaceRoot(#[source] crate::fs::FsError),
    /// Failed to spawn OS command.
    #[error("failed to spawn command")]
    Spawn(#[source] std::io::Error),
    /// Failed to find the right program at a given path.
    #[error("failed to find program")]
    ProgramNotFound {
        #[source]
        e: std::io::Error,
        help: &'static str,
    },
    /// Failed to wait for the command to finish.
    #[error("failed to join running command")]
    Join(#[source] std::io::Error),
    /// Command has already been piped before this attempt.
    #[error("command was already piped")]
    Piped,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum CargoProfile {
    #[default]
    Dev,
    Release,
}

impl CargoProfile {
    pub const fn is_release(self) -> bool {
        matches!(self, Self::Release)
    }

    pub const fn target_folder(self) -> &'static str {
        match self {
            Self::Dev => "debug",
            Self::Release => "release",
        }
    }
}

/// An async command that can be spawned and piped.
pub struct Command {
    inner: tokio::process::Command,
    /// A message to pass to the error in case the program is not found.
    program_help: &'static str,
    name: &'static str,
    piped: bool,
}

impl Command {
    pub fn into_program_action(self) -> ProgramAction {
        ProgramCommand::from(self).into()
    }

    pub fn with_output_to(self, output: ProgramOutputId) -> ProgramAction {
        ProgramCommand::from(self).output_to(output).into()
    }
}

impl Command {
    /// The program to run for for [`Self::pnpm_unchecked`].
    fn pnpm_cmd() -> &'static str {
        cfg!(windows).then_some("pnpm.cmd").unwrap_or("pnpm")
    }

    /// The program to run for [`Self::cargo_unchecked`].
    const fn cargo_cmd() -> &'static str {
        "cargo"
    }

    /// The display name of the command.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Constructs a new command that has not had its stdout and stderr piped.
    /// Does not sanitize the path.
    fn new_unchecked(
        name: &'static str,
        dir: impl AsRef<Path>,
        program: impl AsRef<OsStr>,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
        help: &'static str,
    ) -> Self {
        let mut inner = tokio::process::Command::new(program);
        inner.args(args).current_dir(dir);
        Self {
            name,
            inner,
            piped: false,
            program_help: help,
        }
    }

    /// Constructs a `pnpm` command, to be executed at the given directory.
    /// Does not sanitize the path.
    fn pnpm_unchecked(
        name: &'static str,
        dir: impl AsRef<Path>,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Self {
        let help = "PNpm is a build dependency. See: <https://pnpm.io/>";
        Self::new_unchecked(name, dir, Self::pnpm_cmd(), args, help)
    }

    /// Constructs a `pnpm` command that installs dependencies for the pnpm workspace
    /// in the current working directory.
    pub async fn pnpm_install() -> Result<Self, CommandError> {
        let dir = crate::fs::pnpm_workspace()
            .await
            .map_err(CommandError::PnpmWorkspaceRoot)?;
        let command = Self::pnpm_unchecked("pnpm-install", dir, ["install"]);
        Ok(command)
    }

    /// Constructs a `pnpm` command that starts a Vite dev server.
    pub async fn pnpm_dev() -> Result<Self, CommandError> {
        let dir = crate::fs::pnpm_workspace()
            .await
            .map_err(CommandError::PnpmWorkspaceRoot)?;
        let command = Self::pnpm_unchecked("pnpm-dev", dir, ["dev"]);
        Ok(command)
    }

    /// Constructs a `pnpm` command that builds the `chipbox-frontend` package.
    pub async fn pnpm_build_frontend() -> Result<Self, CommandError> {
        let dir = crate::fs::pnpm_workspace()
            .await
            .map_err(CommandError::PnpmWorkspaceRoot)?;
        let command = Self::pnpm_unchecked(
            "pnpm-build_frontend",
            dir,
            ["--filter", "chipbox-frontend", "build"],
        );
        Ok(command)
    }

    /// Constructs a `cargo` command, to be executed at the given directory.
    /// Does not sanitize the path.
    fn cargo_unchecked(
        name: &'static str,
        dir: impl AsRef<Path>,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Self {
        let help = "Unable to find Cargo. What?";
        Self::new_unchecked(name, dir, Self::cargo_cmd(), args, help)
    }

    /// Constructs a `cargo` command that builds the `chipbox` binary.
    pub async fn cargo_build_chipbox(profile: CargoProfile) -> Result<Self, CommandError> {
        let dir = crate::fs::cargo_workspace()
            .await
            .map_err(CommandError::CargoWorkspaceRoot)?;
        let args = ["build", "-p", "chipbox"]
            .into_iter()
            .chain(profile.is_release().then_some("--release"));
        let command = Self::cargo_unchecked("cargo-build_chipbox", dir, args);
        Ok(command)
    }

    /// Constructs a `cargo` command that generates the `ts-rs` bindings for `chipbox-render`.
    pub async fn cargo_generate_render_bindings() -> Result<Self, CommandError> {
        let dir = crate::fs::cargo_workspace()
            .await
            .map_err(CommandError::CargoWorkspaceRoot)?;
        let args = ["bindings", "-p", "chipbox-render"];
        let command = Self::cargo_unchecked("cargo-bindings_render", dir, args);
        Ok(command)
    }

    /// Constructs a command that runs the compiled `chipbox` binary with the given arguments.
    /// Locates the binary based on the current working directory, which must be a cargo workspace.
    pub async fn chipbox(
        profile: CargoProfile,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Result<Self, CommandError> {
        let help = "Unable to locate the built binary. This is a bug.";
        let dir = crate::fs::cargo_workspace()
            .await
            .map_err(CommandError::CargoWorkspaceRoot)?;
        let process = crate::fs::built_chipbox_binary(profile)
            .await
            .map_err(CommandError::CargoWorkspaceRoot)?;
        let command = Self::new_unchecked("chipbox", dir, process, args, help);
        Ok(command)
    }

    /// Pipes the stdout and stderr of the command.
    /// # Errors
    /// Can only be called once on a command.
    /// If done again, returns [`CommandError::Piped`].
    pub fn make_piped(&mut self) -> Result<(), CommandError> {
        if self.piped {
            return Err(CommandError::Piped);
        }
        self.inner
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        self.piped = true;
        Ok(())
    }

    /// Spawns the command, returning a [`Child`] handle that can be joined.
    pub fn spawn(&mut self) -> Result<Child, CommandError> {
        match self.inner.spawn() {
            Ok(child) => Ok(Child::new(child)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let help = self.program_help;
                Err(CommandError::ProgramNotFound { e, help })
            }
            Err(e) => Err(CommandError::Spawn(e)),
        }
    }
}

impl std::fmt::Display for Command {
    /// Formats the command to be executed as a string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let program = self.inner.as_std().get_program().to_string_lossy();
        let args: String = self
            .inner
            .as_std()
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .intersperse(Cow::Borrowed(" "))
            .collect();
        write!(f, "{program} {args}")
    }
}
