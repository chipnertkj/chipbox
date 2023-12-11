pub use channels::Channels;
pub use meta::ProjectMeta;

mod channels;
mod meta;

use serde::{Deserialize, Serialize};

/// Serializable project data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    /// Project metadata.
    pub meta: ProjectMeta,
    /// Audio channels.
    pub channels: Channels,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            meta: ProjectMeta::new(name),
            channels: Channels::default(),
        }
    }
}
