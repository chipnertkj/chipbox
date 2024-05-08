use super::BackendAppState;
use crate::Settings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Messages sent by the backend app thread
/// in response to frontend queries.
pub enum BackendResponse {
    /// Respond with current `BackendAppState`.
    BackendAppState(BackendAppState),
    /// Respond with current `Settings`.
    Settings(Option<Settings>),
    /// Respond with default `Settings`.
    UseDefaultSettings(Settings),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Messages sent by the frontend app thread requesting information from the backend.
pub enum FrontendRequest {
    /// Query current `BackendAppState`.
    AppState,
    /// Query current `Settings`.
    Settings,
    /// Use default settings and query a copy from the backend.
    UseDefaultSettings,
}
