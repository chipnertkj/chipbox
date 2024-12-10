//! Additional information about a [`project`](crate::project) that isn't directly modifiable
//! by the user.
//!
//! This information is primarily used in
//! [`AnyProject::from_json_str`](crate::project::any::AnyProject::from_json_str)
//! for deserializing a project file into the appropriate
//! version of the format.
//! It must remain stable between different versions of the software,
//! although future versions may add optional fields.
//!
//! See the [`project`](crate::project) module documentation
//! for more information.

use serde::{Deserialize, Serialize};
pub use version::ProjectVersion;

mod version;

/// A stable representation of a project file's metadata.
/// Future versions may add optional fields.
///
/// See the [`meta`](self) module documentation for more information.
#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct ProjectMeta {
    /// The version identifier used for determining
    /// the version of the format to use for deserialization.
    version: ProjectVersion,
}

impl ProjectMeta {
    /// Get the version identifier describing a project.
    pub fn version(&self) -> ProjectVersion {
        self.version
    }

    /// Get the metadata used for the [`v1`](crate::project::v1) project format.
    pub(crate) const fn v1() -> &'static Self {
        &Self {
            version: ProjectVersion::V1,
        }
    }
}
