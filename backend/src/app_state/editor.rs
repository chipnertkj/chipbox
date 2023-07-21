//! Implements the `Editor` application state, responsible for providing
//! an interface for `Project` editing to the user.

pub(crate) mod project; // ok
pub(crate) use self::project::Project;

use super::settings::Settings;

/// An interface for `Project` editing.
/// Contains the selected `Project` and user `Settings`.
#[derive(Debug)]
pub(crate) struct Editor {
    settings: Settings,
    project: Project,
}

impl Editor {
    /// Construct an `Editor` with the specified `Settings` and `Project`.
    pub(crate) fn new(settings: Settings, project: Project) -> Self {
        Self { settings, project }
    }
}
