pub use self::{_private::PaError, raw::RawPaError};

pub type PaResult<T> = Result<T, PaError>;

mod raw {
    use crate::{PaError, portaudio};

    pub trait RawPaError {
        fn ok(self) -> Result<(), PaError>;

        fn ok_and<T>(self, value: T) -> Result<T, PaError>
        where
            Self: Sized,
        {
            self.ok().and(Ok(value))
        }

        fn ok_and_then<T, F>(self, f: F) -> Result<T, PaError>
        where
            Self: Sized,
            F: FnOnce() -> T,
        {
            self.ok().map(|()| f())
        }
    }

    impl RawPaError for portaudio::PaError {
        fn ok(self) -> Result<(), PaError> {
            match self {
                portaudio::PaErrorCode_paNoError => Ok(()),
                err => Err(
                    // SAFETY: just checked for `PaErrorCode_paNoError`.
                    unsafe { PaError::from_raw_unchecked(err) },
                ),
            }
        }
    }
}

mod _private {
    //! Implementation details, separated for impl (in)visibility.

    use crate::portaudio;

    /// An error returned by the library.
    /// # Type safety
    /// The error cannot be safely constructed from a non-error code
    /// ([`portaudio::PaErrorCode_paNoError`]).
    #[derive(Debug, thiserror::Error, Clone, Copy, Eq, PartialEq)]
    #[error("{}", self.description().to_string_lossy())]
    pub struct PaError(portaudio::PaError);

    impl PaError {
        /// # Type safety
        /// The error may not be constructed from a non-error code
        /// like [`portaudio::PaErrorCode_paNoError`].
        #[must_use]
        pub(crate) const unsafe fn from_raw_unchecked(raw: portaudio::PaError) -> Self {
            Self(raw)
        }

        #[must_use]
        pub fn description(self) -> &'static std::ffi::CStr {
            // SAFETY: just returns a static cstr.
            let ptr = unsafe { portaudio::Pa_GetErrorText(self.0) };
            // SAFETY: `Pa_GetErrorText` returns a static null-terminated string pointer.
            unsafe { std::ffi::CStr::from_ptr(ptr) as &'static std::ffi::CStr }
        }
    }
}
