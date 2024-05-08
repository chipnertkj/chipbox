pub use edit_state::EditState;
pub mod edit_state;

use crate::common;
use common::app::{AwaitConfigReason, BackendAppState};
use common::Settings;

#[derive(Default)]
/// Backend application state.
pub enum AppState {
    #[default]
    /// Backend is in the process of reading user config.
    ReadingSettings,
    /// Settings read has been attempted, but no valid configuration was found.
    AwaitConfig { reason: AwaitConfigReason },
    /// Backend is awaiting commands from the frontend.
    Idle { settings: Settings },
    /// Backend is ready to edit a project.
    Edit {
        inner: Box<EditState>,
        settings: Settings,
    },
}

/// Initialize app state from an optional configuration.
impl From<Option<Settings>> for AppState {
    fn from(settings_opt: Option<Settings>) -> Self {
        match settings_opt {
            Some(settings) => AppState::Idle { settings },
            None => AppState::AwaitConfig {
                reason: AwaitConfigReason::NoConfig,
            },
        }
    }
}

/// Convert app state to a minimal, serializable version.
impl From<&AppState> for BackendAppState {
    fn from(app: &AppState) -> Self {
        match app {
            AppState::ReadingSettings => BackendAppState::ReadingSettings,
            AppState::AwaitConfig { ref reason } => {
                BackendAppState::AwaitConfig {
                    reason: reason.clone(),
                }
            }
            AppState::Idle { .. } => BackendAppState::Idle,
            AppState::Edit { .. } => BackendAppState::Editor,
        }
    }
}
