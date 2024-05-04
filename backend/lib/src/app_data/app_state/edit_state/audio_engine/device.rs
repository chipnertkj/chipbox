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
            Error::Disconnected(err) => {
                write!(f, "device was disconnected: {err}")
            }
            Error::Other(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NoDefault => None,
            Error::NoMatch => None,
            Error::Disconnected(err) => Some(err.as_ref()),
            Error::Other(err) => Some(err.as_ref()),
        }
    }
}
