use std::process::ExitStatus;

use miette::{Context as _, IntoDiagnostic as _};

use crate::command::CommandError;

/// A running async command that can be joined.
/// See [`Command::spawn`].
///
/// [`Command::spawn`]: crate::command::Command::spawn
pub struct Child {
    inner: tokio::process::Child,
}

impl Child {
    /// Constructs a new [`Command`] handle.
    ///
    /// [`Command`]: crate::command::Command
    pub(super) const fn new(inner: tokio::process::Child) -> Self {
        Self { inner }
    }

    /// Waits for the command to finish, returning the exit status.
    pub async fn join(&mut self) -> Result<ExitStatus, CommandError> {
        self.inner.wait().await.map_err(CommandError::Join)
    }

    /// Awaits the handle to finish and returns the exit status.
    pub async fn await_finish(mut self) -> miette::Result<ExitStatus> {
        self.join()
            .await
            .into_diagnostic()
            .wrap_err("await command finish")
    }

    /// Takes the piped stdout stream from the command.
    pub const fn take_stdout(&mut self) -> Option<tokio::process::ChildStdout> {
        self.inner.stdout.take()
    }

    /// Takes the piped stderr stream from the command.
    pub const fn take_stderr(&mut self) -> Option<tokio::process::ChildStderr> {
        self.inner.stderr.take()
    }
}
