use std::path::PathBuf;

#[derive(Debug)]
pub struct Error {
    pub e: std::io::Error,
    pub path: PathBuf,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.e)
    }
}
