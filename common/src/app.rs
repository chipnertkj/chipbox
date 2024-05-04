use crate::Settings;
use serde::{Deserialize, Serialize};

/// Messages sent by the backend app thread.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum BackendMsg {
    /// Backend app is currently reading user config.
    ReadingSettings,
    /// Response to a frontend request.
    Response(BackendResponse),
}

impl BackendMsg {
    /// JS Event name used by the frontend.
    pub const fn event_name() -> &'static str {
        "chipbox-app-message"
    }
}

/// Messages sent by the backend app thread in response to frontend queries.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum BackendResponse {
    /// Respond with current `BackendAppState`.
    BackendAppState(BackendAppState),
    /// Respond with current `Settings`.
    Settings(Option<Settings>),
    /// Respond with default `Settings`.
    UseDefaultSettings(Settings),
}

/// Messages sent by the frontend app thread.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FrontendMsg {
    /// Query information from the backend.
    Request(FrontendRequest),
}

/// Messages sent by the frontend app thread requesting information from the backend.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FrontendRequest {
    /// Query current `BackendAppState`.
    BackendAppState,
    /// Query current `Settings`.
    Settings,
    /// Use default settings and query a copy from the backend.
    UseDefaultSettings,
}

/// The reason why the user config is not ready.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Default)]
pub enum AwaitConfigReason {
    #[default]
    /// This is the first time the application has been started.
    ///
    /// The user has not yet configured the application.
    NoConfig,
}

/// Minimal description of the current state of the backend.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Default)]
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
