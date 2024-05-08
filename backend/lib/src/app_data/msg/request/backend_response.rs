use crate::app_data::app_state::AppState;
use crate::app_data::AppData;
use crate::common;
use common::app::msg::request::BackendResponse;

/// Reply with current state.
pub(super) fn app_state(app_data: &mut AppData) -> BackendResponse {
    // Get state and convert it to `BackendAppState`.
    let app_state = (&app_data.state).into();

    // Prepare response.
    BackendResponse::BackendAppState(app_state)
}

/// Reply with current settings.
pub(super) fn settings(app_data: &mut AppData) -> BackendResponse {
    // Get settings.
    let settings_opt = match app_data.state {
        AppState::Idle { ref settings } => Some(settings.clone()),
        _ => None,
    };

    // Prepare response.
    BackendResponse::Settings(settings_opt)
}

/// Set default settings and reply with a copy.
pub(super) fn use_default_settings(app_data: &mut AppData) -> BackendResponse {
    // Apply and return settings.
    match app_data.state {
        // Fail gracefully if in the middle of reading settings.
        AppState::ReadingSettings => {
            tracing::error!("Frontend attempted to set settings to default while backend was still reading config.");
        }
        // Change state to idle if awaiting config.
        AppState::AwaitConfig { .. } => {
            app_data.state = AppState::Idle {
                settings: Default::default(),
            };
        }
        // Modify if idle.
        AppState::Idle { ref mut settings } => {
            *settings = Default::default();
        }
        // Modify if editing a project.
        AppState::Edit {
            ref mut settings, ..
        } => {
            *settings = Default::default();
        }
    }

    // Prepare response.
    BackendResponse::UseDefaultSettings(Default::default())
}
