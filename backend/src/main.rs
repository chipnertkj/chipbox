//! The binary for the backend application.
//!
//! Defines the entry point for platforms other than mobile.
//! See [`main`] for more information.

// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Inner entry point code resides in [chipbox_lib::run].
/// This function is only relevant for platforms other than mobile.
///
/// This is done for mobile platform compatibility, see
/// [here](https://v2.tauri.app/start/migrate/from-tauri-1/#preparing-for-mobile).
fn main() -> color_eyre::Result<()> {
    chipbox_lib::run()
}
