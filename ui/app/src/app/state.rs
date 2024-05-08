use super::backend_query::BackendQueryState;
use super::home::HomeState;
use super::setup::SetupState;
use crate::common;
use common::app::{AwaitConfigReason, BackendAppState};

impl From<BackendAppState> for AppState {
    fn from(value: BackendAppState) -> Self {
        match value {
            BackendAppState::ReadingSettings => {
                Self::BackendQuery(BackendQueryState::ReadSettings)
            }
            BackendAppState::AwaitConfig { reason } => match reason {
                AwaitConfigReason::NoConfig => Self::Setup(SetupState::First),
                AwaitConfigReason::Error(err) => {
                    Self::Setup(SetupState::Error(err))
                }
            },
            BackendAppState::Idle => Self::Home(HomeState::Welcome),
            BackendAppState::Editor => todo!(),
        }
    }
}

#[derive(PartialEq)]
/// Frontend application state.
pub enum AppState {
    /// Backend app thread channel was closed.
    /// Communication with the backend is no longer possible.
    BackendClosed,
    /// Frontend is waiting for backend app thread response.
    BackendQuery(BackendQueryState),
    /// Frontend is in setup mode.
    /// This is where the user can modify application configuration.
    Setup(SetupState),
    /// User must select a project to continue to the editor.
    Home(HomeState),
}

impl Default for AppState {
    fn default() -> Self {
        Self::BackendQuery(Default::default())
    }
}
