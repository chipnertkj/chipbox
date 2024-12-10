//! An interface for performing operations on a [`Project`].

use super::Project;

/// Defines operations that can be performed on a project
/// in order to modify its contents.
#[derive(Debug)]
pub enum Cmd {
    /// Add a new instrument channel to the song.
    AddInstrumentChannel,
}

/// A manager for a [`Project`] that provides a limited set of methods for
/// modifying the project's contents.
#[derive(Debug)]
pub struct ProjectManager {
    project: Project,
}

impl ProjectManager {
    /// Create a new [`ProjectManager`] for a [`Project`].
    pub fn new(project: Project) -> Self {
        Self { project }
    }

    /// Execute a command on the project.
    pub fn execute(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::AddInstrumentChannel => todo!(),
        }
    }
}
