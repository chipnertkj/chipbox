use crate::{PaResult, RawPaError as _, portaudio};

pub struct Instance {}

impl Instance {
    pub fn new() -> PaResult<Self> {
        // SAFETY: must call `Pa_Terminate` on drop.
        // May be called repeatedly (uses incremental reference counting).
        // See `Drop` implemenation.
        unsafe { portaudio::Pa_Initialize() }.ok_and_then(|| Self {})
    }

    /// Deallocates all resources used by the library.
    /// # Safety
    /// May be called repeatedly (uses incremental reference counting).
    /// Must be matched with every call to [`portaudio::Pa_Initialize`] that returns
    /// [`portaudio::PaErrorCode_paNoError`].
    unsafe fn terminate() -> PaResult<()> {
        // SAFETY: containing function describes safety conditions.
        unsafe { portaudio::Pa_Terminate() }.ok()
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        // SAFETY: Matched with every call to `Pa_Initialize`.
        // We discard the result as aborting here could leak resources.
        let _result = unsafe { Self::terminate() };
    }
}
