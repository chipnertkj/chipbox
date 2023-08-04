//! The `handler` module contains the `add_to_builder` function.
//! It is responsible for generating an invoke handler and adding it to a `tauri::Builder`.

use crate::state::{__cmd__state, state};

/// Generates an invoke handler and adds it to a `tauri::Builder`.
///
/// The handler contains commands required for interop between the backend and frontend applications.
pub fn add_to_builder<R>(builder: tauri::Builder<R>) -> tauri::Builder<R>
where
    R: tauri::Runtime,
{
    builder.invoke_handler(tauri::generate_handler![state])
}
