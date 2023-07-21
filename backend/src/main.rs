//! Chipbox `tauri` application, from now on referred to as the backend.
//!
//! The provided code documentation is minimal at best.
//! It should stay that way, as its primary purpose is
//! not to guide users of the application - rather,
//! it serves as context for developers.
//! It is supposed to increase code readability.
//!
//!
//! This binary crate is responsible for:
//! - setting up `tauri`,
//! - providing commands used for frontend communication,
//! - implementing functionality that may benefit from running natively,
//! such as DSP and I/O.
//!
//! # Inline comments for external file modules
//! For clarity, every external file module, considered finished
//! as per *current* project requirements, should have an inline comment,
//! on the line declaring the module - right next to it - containing "ok".

#![feature(never_type)]
#![feature(try_find)]
#![feature(iter_repeat_n)]
// Enable Windows application subsystem on release.
// This prevents an additional console window from showing up on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Lints disabled for development purposes:
#![allow(dead_code)]

pub(crate) mod app_state;
pub(crate) mod path; // ok

use app_state::AppStateManaged;
use color_eyre::eyre;
use std::error;
use tauri::async_runtime;
use tracing_subscriber::util::SubscriberInitExt as _;

/// The `tauri::Builder` application setup hook.
/// Used to asynchronously initialize `tauri::App`-managed application state.
fn setup(app: &mut tauri::App) -> Result<(), Box<dyn error::Error>> {
    let rt = async_runtime::handle();
    rt.spawn(AppStateManaged::setup(app.handle()));
    Ok(())
}

/// Callback function called at the end of every `tauri::App` event loop iteration.
fn run(_app_handle: &tauri::AppHandle, _event: tauri::RunEvent) {}

/// Application entry point.
/// Initialize `color_eyre`, `tracing` and `tauri`.
fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::FmtSubscriber::default().init();
    let app = tauri::Builder::default()
        .setup(setup)
        .manage(AppStateManaged::default())
        .invoke_handler(tauri::generate_handler![])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .build(tauri::generate_context!())
        .expect("error while building `tauri::App`");
    app.run(run);
    Ok(())
}
