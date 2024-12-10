//! This module lists all versions of the project format
//! that can be interpreted by the current version of
//! the software.

use crate::project::latest;
use serde::{Deserialize, Serialize};

/// Version of the project format used by a project.
///
/// This metadata value is used for determining the
/// version of the format that should be used for
/// the deserialization of a project file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, derive_more::Display)]
pub enum ProjectVersion {
    /// The first version of the project format.
    #[display("v1")]
    V1,
    /// An unsupported version of the project format.
    /// This is only used for signalling the issue to the user.
    #[serde(untagged)]
    Unrecognized,
}

impl ProjectVersion {
    /// The latest known version of the project format.
    ///
    /// Determined by the [`latest`] module re-export.
    pub const fn latest() -> Self {
        latest::Project::project_meta().version
    }

    /// Check if this version is the latest version
    /// of the project format that can be used.
    pub fn is_latest(&self) -> bool {
        self == &Self::latest()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Ensures the latest version is actually the latest.
    #[test]
    fn latest_is_latest() {
        assert!(ProjectVersion::latest().is_latest());
    }
}
