pub mod tree_location;
pub use self::tree_location::TreeLocation;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ProjectManagement {
    pub project_tree_location: TreeLocation,
}
