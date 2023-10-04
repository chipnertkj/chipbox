use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct ProjectPath {
    pub name: String,
    pub group: Option<String>,
}

impl ProjectPath {
    pub fn new(name: String, group: Option<String>) -> Self {
        Self { name, group }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub project_path: ProjectPath,
    pub author: String,
    pub icon_path: Option<PathBuf>,
    pub date: chrono::DateTime<chrono::Utc>,
}
