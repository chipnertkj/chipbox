use serde::{Deserialize, Serialize};

/// Metadata about a project.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    /// Name of the project.
    pub name: String,
    /// Description of the project.
    pub description: Option<String>,
    /// Name of the author of the project.
    pub author: Option<String>,
    /// Date when the project was created.
    pub creation_date: chrono::DateTime<chrono::Utc>,
    /// `None` if the project hasn't been modified since creation.
    pub modification_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl ProjectMeta {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            author: None,
            creation_date: chrono::Utc::now(),
            modification_date: None,
        }
    }
}
