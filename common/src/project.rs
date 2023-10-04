pub use channels::Channels;
pub use meta::{ProjectMeta, ProjectPath};

mod channels;
mod meta;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    channels: Channels,
}
