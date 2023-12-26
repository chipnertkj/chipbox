//! The `handler` module contains the `add_to_builder` function.
//! It is responsible for generating an invoke handler and adding it to a `tauri::Builder`.

use crate::msg::{__cmd__frontend_msg, frontend_msg};

pub trait BuilderGlue<R: tauri::Runtime> {
    /// Generates an invoke handler and adds it to a `tauri::Builder`.
    ///
    /// The handler contains commands required for backend and frontend interop.
    fn glue_invoke_handler(self) -> Self;
}

/// Implement `BuilderGlue` for `tauri::Builder`.
impl<R: tauri::Runtime> BuilderGlue<R> for tauri::Builder<R> {
    fn glue_invoke_handler(self) -> Self {
        self.invoke_handler(tauri::generate_handler![frontend_msg])
    }
}
