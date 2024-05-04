use std::path::PathBuf;

#[derive(Debug)]
pub struct SerdeError {
    pub err: serde_json::Error,
    pub path: PathBuf,
}

impl std::error::Error for SerdeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.err)
    }
}

impl std::fmt::Display for SerdeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}
