//! Implements the `Welcome` application state, responsible for project management
//! and selection, as well as the `project_tree` module.
pub mod project_tree; // ok

use super::Settings;
use serde::{Deserialize, Serialize};

/// `Project` selection and management.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Welcome {
    settings: Settings,
}

impl Welcome {
    /// Construct the `Welcome` state with the specified `Settings`.
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }
}
