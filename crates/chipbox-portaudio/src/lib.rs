pub(crate) use chipbox_portaudio_sys as portaudio;

pub(crate) use self::error::RawPaError;
pub use self::{
    error::{PaError, PaResult},
    instance::Instance,
};

mod error;
mod instance;
