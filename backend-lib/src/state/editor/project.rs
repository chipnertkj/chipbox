//! Implements a deserialized, virtual representation of a chipbox project.

use serde::{Deserialize, Serialize};

/// Deserialized, virtual representation of a chipbox project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {}
