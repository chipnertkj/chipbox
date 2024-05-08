pub mod msg;

use cowstr::CowStr;
use serde::{Deserialize, Serialize};

/// The reason why the user config is not ready.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum AwaitConfigReason {
    #[default]
    /// This is the first time the application has been started.
    ///
    /// The user has not yet configured the application.
    NoConfig,
    /// An error occurred while reading the user config.
    Error(CowStr),
}

impl std::error::Error for AwaitConfigReason {}

impl std::fmt::Display for AwaitConfigReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AwaitConfigReason::NoConfig => {
                write!(f, "No user configuration found")
            }
            AwaitConfigReason::Error(err) => {
                write!(f, "Error reading user configuration: {}", err)
            }
        }
    }
}

/// Minimal description of the current state of the backend.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum BackendAppState {
    /// Backend is currently reading user config.
    #[default]
    ReadingSettings,
    /// User config has been read, but no valid configuration was found.
    AwaitConfig { reason: AwaitConfigReason },
    /// User config was read and is valid.
    Idle,
    /// Backend is ready to edit a project.
    Editor,
}
