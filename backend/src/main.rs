//! Chipbox `tauri` application backend.

use color_eyre::eyre;
use glue::handler::BuilderGlue as _;
use std::sync::Arc;
use tauri::{async_runtime, Manager};
use tracing_subscriber::util::SubscriberInitExt as _;
use {chipbox_backend_lib as backend_lib, chipbox_glue as glue};

/// Construct and configure a `tauri::App`.
fn tauri_app() -> tauri::App {
    let window_plugin = tauri_plugin_window_state::Builder::default().build();
    // Create builder.
    tauri::Builder::default()
        .manage::<backend_lib::ManagedApp>(Default::default())
        .plugin(window_plugin)
        .glue_invoke_handler() // See `glue::handler::BuilderGlue`.
        .build(tauri::generate_context!())
        .expect("error while building `tauri::App`")
}

/// Event handler callback for `tauri::App`.
fn run(tauri_app: &tauri::AppHandle, event: tauri::RunEvent) {
    if let tauri::RunEvent::Ready = event {
        let state = tauri_app.state::<backend_lib::ManagedApp>();
        ready(Arc::clone(&state));
    }
}

/// Called when the application starts up.
/// Asynchronously load settings and continue to `backend_lib::App::Setup` state.
fn ready(app: backend_lib::ManagedApp) {
    let rt = async_runtime::handle();
    rt.spawn(async move {
        let setup = backend_lib::Setup::read_settings().await;
        let mut guard = app.lock().await;
        *guard = backend_lib::App::Setup(setup);
    });
}

/// Application entry point.
/// Initialize `color_eyre`, `tracing-subscriber` and `tauri`.
fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::FmtSubscriber::default().init();
    tauri_app().run(run);
    Ok(())
}
