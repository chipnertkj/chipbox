//! Implements the `Editor` application state, responsible for providing
//! an interface for `Project` editing to the user.

// ok
pub mod project;
pub use self::project::Project;

use super::settings::Settings;
use serde::{Deserialize, Serialize};

/// An interface for `Project` editing.
/// Contains the selected `Project` and user `Settings`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Editor {
    settings: Settings,
    project: Project,
}

impl Editor {
    /// Construct an `Editor` with the specified `Settings` and `Project`.
    pub fn new(settings: Settings, project: Project) -> Self {
        Self { settings, project }
    }
}
