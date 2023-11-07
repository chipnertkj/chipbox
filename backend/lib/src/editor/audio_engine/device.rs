#[derive(Debug)]
pub enum Error {
    NoDefault,
    NoMatch,
    Disconnected(Box<dyn std::error::Error>),
    Other(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoDefault => write!(
                f,
                "unable to find a default device or no default device is set"
            ),
            Error::NoMatch => write!(f, "no matching device"),
            Error::Disconnected(e) => {
                write!(f, "device was disconnected: {e}")
            }
            Error::Other(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NoDefault => None,
            Error::NoMatch => None,
            Error::Disconnected(e) => Some(e.as_ref()),
            Error::Other(e) => Some(e.as_ref()),
        }
    }
}
