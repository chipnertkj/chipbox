use std::path::PathBuf;

pub struct ProjectPath {
    pub name: String,
    pub group: Option<String>,
}

impl ProjectPath {
    pub fn new(name: String, group: Option<String>) -> Self {
        Self { name, group }
    }
}

pub struct ProjectMeta {
    pub project_path: ProjectPath,
    pub author: String,
    pub icon_path: Option<PathBuf>,
    pub date: chrono::DateTime<chrono::Utc>,
}
