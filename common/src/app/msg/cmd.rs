use crate::app::AwaitConfigReason;
use crate::Settings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum BackendCmd {
    /// Inform the frontend that the backend app
    /// is currently reading user config.
    ReadingSettings,
    /// Update the config of the frontend app.
    UpdateSettings(Result<Settings, AwaitConfigReason>),
}
