//! Implements the application state during the first-time configuration process.

use super::Settings;

/// Application state during the first-time configuration process.
#[derive(Debug, Default)]
pub(crate) struct FirstTimeSetup {
    settings: Settings,
}
