use self::app_state::AppState;
use crate::{common, AppThread, ThreadMsg};
use common::app::{BackendMsg, BackendResponse, FrontendMsg, FrontendRequest};

mod app_state;

/// All data required for managing the application.
///
/// Includes its state and the handle to the Tauri application.
pub struct AppData {
    pub state: AppState,
    tauri_app: tauri::AppHandle,
}

impl AppData {
    /// Create new app data.
    pub fn new(tauri_app: tauri::AppHandle) -> Self {
        Self {
            state: Default::default(),
            tauri_app,
        }
    }

    /// Associated Tauri application handle.
    pub fn tauri_app(&self) -> &tauri::AppHandle {
        &self.tauri_app
    }

    /// Handles a message from the parent thread.
    /// Returns `true` if the app should quit.
    pub fn handle_msg(&mut self, msg: ThreadMsg) -> bool {
        match msg {
            // Handle frontend message.
            ThreadMsg::Frontend(msg) => self.handle_frontend_msg(msg),
            // Quit.
            ThreadMsg::Exit => return true,
        };
        false
    }

    /// Handles messages from the frontend.
    fn handle_frontend_msg(&mut self, msg: FrontendMsg) {
        match msg {
            FrontendMsg::Request(request) => {
                self.handle_frontend_request(request)
            }
        }
    }

    /// Handles the request and sends a response to the frontend.
    fn handle_frontend_request(&mut self, request: FrontendRequest) {
        // Handle request and prepare response.
        let response = match request {
            FrontendRequest::BackendAppState => {
                self.backend_app_state_response()
            }
            FrontendRequest::Settings => self.backend_settings_response(),
            FrontendRequest::UseDefaultSettings => {
                self.use_default_settings_response()
            }
        };

        // Send response.
        AppThread::send_message(
            &self.tauri_app,
            BackendMsg::Response(response),
        );
    }

    /// Reply with current state.
    fn backend_app_state_response(&mut self) -> BackendResponse {
        // Get state and convert it to `BackendAppState`.
        let app_state = (&self.state).into();

        // Prepare response.
        BackendResponse::BackendAppState(app_state)
    }

    /// Reply with current settings.
    fn backend_settings_response(&mut self) -> BackendResponse {
        // Get settings.
        let settings_opt = match self.state {
            AppState::Idle { ref settings } => Some(settings.clone()),
            _ => None,
        };

        // Prepare response.
        BackendResponse::Settings(settings_opt)
    }

    /// Set default settings and reply with a copy.
    pub fn use_default_settings_response(&mut self) -> BackendResponse {
        // Apply and return settings.
        match self.state {
            // Fail gracefully if in the middle of reading settings.
            AppState::ReadingSettings => {
                tracing::error!("Frontend attempted to set settings to default while backend was still reading config.");
            }
            // Change state to idle if awaiting config.
            AppState::AwaitConfig { .. } => {
                self.state = AppState::Idle {
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
}
