//! User configuration.

use serde::{Deserialize, Serialize};

/// User configuration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    /// Unrecognized keys from the settings file.
    ///
    /// Kept so that fields removed in newer versions of the software
    /// can still be serialized back into the same file.
    #[serde(flatten)]
    unrecognized: serde_json::Value,
}
