use std::path::PathBuf;

#[derive(Debug)]
pub struct IoError {
    pub err: std::io::Error,
    pub path: PathBuf,
}

impl std::error::Error for IoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.err)
    }
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at path `{}`", self.err, self.path.display())
    }
}
