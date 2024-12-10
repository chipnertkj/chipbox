//! Shared library entry point for the application.
//!
//! See [`run`] and the main function in the binary for more information.

use crate::tracing_layers;
use chipbox_glue::{handler::BuilderGlue as _, loaded_project::LoadedProject};

/// Shared library entry point for the application.
///
/// See [here](https://v2.tauri.app/start/migrate/from-tauri-1/#preparing-for-mobile)
/// for more information.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> color_eyre::Result<()> {
    // Must be first, installs panic and error report hooks.
    color_eyre::install()?;
    let _tracing_guard = tracing_layers::init()?;

    // Use [`tauri`] as the application framework.
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(LoadedProject::default())
        .glue_invoke_handler()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
