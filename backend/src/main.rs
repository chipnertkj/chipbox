//! Chipbox `tauri` application backend.

use color_eyre::eyre;
use glue::handler::BuilderGlue as _;

use {chipbox_backend_lib as backend_lib, chipbox_glue as glue};

mod app_thread;
mod tracing_setup;

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
        // Unrecoverable error.
        .expect("error while building `tauri::App`")
}

/// Tauri app is exiting.
/// Called on `tauri::RunEvent::Exit`
fn exit(
    managed_app_thread_handle: &mut app_thread::ManagedJoinHandle,
    app: &tauri::AppHandle,
) {
    tracing::trace!("Tauri app is exiting as requested.");

    // Close the backend lib app thread.
    let rt = tauri::async_runtime::handle();
    rt.block_on(app_thread::close(managed_app_thread_handle, app));
}

/// Tauri app is ready.
/// Called on `tauri::RunEvent::Ready`.
fn ready(
    managed_app_thread_handle: &mut app_thread::ManagedJoinHandle,
    app: &tauri::AppHandle,
) {
    // Start the backend lib app thread.
    let rt = tauri::async_runtime::handle();
    rt.block_on(app_thread::start(managed_app_thread_handle, app));

    // All ok.
    tracing::trace!("Tauri app is ready.");
}

/// Tauri app event handler.
fn run(
    managed_app_thread_handle: &mut app_thread::ManagedJoinHandle,
    app: &tauri::AppHandle,
    event: tauri::RunEvent,
) {
    match event {
        tauri::RunEvent::Ready => ready(managed_app_thread_handle, app),
        tauri::RunEvent::Exit => exit(managed_app_thread_handle, app),
        _ => {}
    }
}

/// Application entry point.
fn main() -> eyre::Result<()> {
    // Install color-eyre.
    color_eyre::install()?;

    // Initialize tracing.
    let rt = tauri::async_runtime::handle();
    let _guard = tracing_setup::init(rt)?;

    // Create managed handle for the app thread.
    let mut managed_app_thread_handle =
        app_thread::ManagedJoinHandle::default();

    // Start tauri app.
    tauri_app()
        .run(move |app, event| run(&mut managed_app_thread_handle, app, event));

    // Tauri calls `std::process::exit(0)` after `RunEvent::Exit`.
    // Modify accordingly if changed in the future.
    // It is expected behavior as of now.
    eyre::bail!(
        "Process should've terminated after `RunEvent::Exit` but did not"
    );
}
