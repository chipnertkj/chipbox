//! Chipbox `tauri` application backend.

use color_eyre::eyre;
use glue::handler::BuilderGlue as _;
use tauri::{async_runtime, Manager};
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
    match event {
        tauri::RunEvent::Ready => {
            let state = tauri_app.state::<backend_lib::ManagedApp>();
            ready(state.inner().clone());
        }
        tauri::RunEvent::Exit => {
            exit();
        }
        _ => {}
    }
}

/// Called when the application starts up.
/// Asynchronously load settings and continue to `backend_lib::App::Setup` state.
fn ready(managed_app: backend_lib::ManagedApp) {
    // Init STREAMS thread-local storage.
    backend_lib::stream_handle::init_streams();
    // Read settings.
    let rt = async_runtime::handle();
    rt.spawn(async move {
        let setup = backend_lib::Setup::read_settings().await;
        let mut guard = managed_app.arc.lock().await;
        *guard = backend_lib::App::Setup(setup);
    });
}

// Called when the application exits.
// Clears STREAMS thread-local storage.
fn exit() {
    // Clear thread-local storage on main thread.
    // This would be done anyways due to the `Drop` implementation,
    // but dropping streams during exit seems to cause a panic.
    // This manually gets rid of the streams beforehand.
    backend_lib::stream_handle::clear_streams();
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
    // Start app.
    tauri_app().run(run);
    // Tauri calls `std::process::exit(0)` after `RunEvent::Exit`.
    // Modify accordingly if changed in the future.
    // This is expected behavior as of now.
    unreachable!("Process should've terminated by now.")
}
