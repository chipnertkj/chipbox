//! Chipbox `tauri` application backend.

use color_eyre::eyre;
use glue::handler::BuilderGlue as _;

use {chipbox_backend_lib as backend_lib, chipbox_glue as glue};

mod app_thread;

/// Construct and configure the `tauri_plugin_window_state` plugin.
fn window_plugin<R>() -> tauri::plugin::TauriPlugin<R>
where
    R: tauri::Runtime,
{
    tauri_plugin_window_state::Builder::default().build()
}

/// Construct and configure a `tauri::App`.
fn tauri_app() -> tauri::App {
    // Create builder.
    tauri::Builder::default()
        // Add window plugin.
        .plugin(window_plugin())
        // See `glue::handler::BuilderGlue`.
        .glue_invoke_handler()
        // Use project context.
        .build(tauri::generate_context!())
        // Something went wrong while building the app.
        .expect("error while building `tauri::App`")
}

/// Tauri app is exiting.
/// Called on `tauri::RunEvent::Exit`
/// Close the backend lib app thread.
fn exit(app: &tauri::AppHandle) {
    tracing::trace!("Tauri app is exiting.");
    let rt = tauri::async_runtime::handle();
    rt.block_on(app_thread::close(app));
}

/// Tauri app is ready.
/// Called on `tauri::RunEvent::Ready`.
/// Start the backend lib app thread.
fn ready(app: &tauri::AppHandle) {
    let rt = tauri::async_runtime::handle();
    rt.block_on(app_thread::start(app));
    // All ok.
    tracing::trace!("Tauri app is ready.");
}

/// Tauri app event handler.
fn run(app: &tauri::AppHandle, ev: tauri::RunEvent) {
    match ev {
        tauri::RunEvent::Ready => ready(app),
        tauri::RunEvent::Exit => exit(app),
        _ => {}
    }
}

/// Application entry point.
/// Initialize `color_eyre`, `tracing-subscriber` and `tauri`.
fn main() -> eyre::Result<()> {
    // Install color-eyre.
    color_eyre::install()?;
    // Install subscriber.
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // Start tauri app.
    tauri_app().run(run);
    // Tauri calls `std::process::exit(0)` after `RunEvent::Exit`.
    // Modify accordingly if changed in the future.
    // This is expected behavior as of now.
    unreachable!("Process should've terminated by now.")
}
