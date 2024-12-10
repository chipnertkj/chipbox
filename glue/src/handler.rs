//! This module defines [`BuilderGlue`], an extension trait for [`tauri::Builder`].
//!
//! The trait allows you to add commands defined in the [`cmd`](crate::cmd) module to a [`tauri`] application.
use crate::cmd::{create_project::backend::*, loaded_project::backend::*};

/// Extension trait for [`tauri::Builder`].
///
/// This trait is responsible for generating a `tauri::ipc::Invoke` handler and adding it to a [`tauri::Builder`].
/// The generated handler contains commands defined by the [`cmd`](crate::cmd) module.
/// They are required for backend-frontend interoperability.
pub trait BuilderGlue<R: tauri::Runtime> {
    /// Modifies `Self` to use a handler supplied by the implementation of this function.
    ///
    /// The generated handler contains commands defined by the [`cmd`](crate::cmd) module.
    fn glue_invoke_handler(self) -> Self;
}

/// Any commands added to [`crate::cmd`] must be added here for `tauri` to recognize them!
impl<R: tauri::Runtime> BuilderGlue<R> for tauri::Builder<R> {
    fn glue_invoke_handler(self) -> Self {
        self.invoke_handler(tauri::generate_handler![create_project, loaded_project])
    }
}
