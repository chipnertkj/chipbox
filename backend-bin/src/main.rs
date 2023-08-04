//! Chipbox `tauri` application backend.
//!
//! This binary crate sets up `tauri` and the application defined in `chipbox-backend-lib`.
//!
//! # Inline comments for external file modules
//! For clarity, every finished external file module should have an inline comment
//! right next to it, containing "ok".

use chipbox_backend_lib::state::ManagedState;
use color_eyre::eyre;
use std::error;
use tauri::async_runtime;
use tracing_subscriber::util::SubscriberInitExt as _;

/// The `tauri::Builder` application setup hook.
/// Used to asynchronously initialize `tauri::App`-managed application state.
fn setup(app: &mut tauri::App) -> Result<(), Box<dyn error::Error>> {
    let rt = async_runtime::handle();
    rt.spawn(ManagedState::setup(app.handle()));
    Ok(())
}

/// Construct and configure a `tauri::App` with commands and managed state.
fn app() -> tauri::App {
    let builder = tauri::Builder::default()
        .setup(setup)
        .manage(ManagedState::default())
        .plugin(tauri_plugin_window_state::Builder::default().build());
    chipbox_glue::handler::add_to_builder(builder)
        .build(tauri::generate_context!())
        .expect("error while building `tauri::App`")
}

/// Callback function called at the end of every `tauri::App` event loop iteration.
fn run(_app_handle: &tauri::AppHandle, _event: tauri::RunEvent) {}

/// Application entry point.
/// Initialize `color_eyre`, `tracing` and `tauri`.
fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::FmtSubscriber::default().init();
    app().run(run);
    Ok(())
}
