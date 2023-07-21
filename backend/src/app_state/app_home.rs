//! Implements the `Home` application state, responsible for project management
//! and selection, as well as the `project_tree` module.
pub(crate) mod project_tree; // ok

use super::Settings;

/// `Project` selection and management.
#[derive(Debug)]
pub(crate) struct AppHome {
    settings: Settings,
}

impl AppHome {
    /// Construct the `AppHome` state with the specified `Settings`.
    pub(crate) fn new(settings: Settings) -> Self {
        Self { settings }
    }
}
