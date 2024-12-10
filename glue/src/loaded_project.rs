//! Tauri application state for a project that may be loaded and unloaded.

use chipbox_common::project::latest::Project;
use parking_lot::Mutex;

/// A wrapper around a [`Project`] that is stored in the [tauri application's](tauri::App) state manager.
///
/// The project is an [`Option`] because the application does not need to have a project loaded to run.
/// This may happen when the application is first started, for example.
///
/// The option is wrapped in a [`Mutex`] to enable concurrent mutation.
#[derive(Default, Debug)]
pub struct LoadedProject(pub Mutex<Option<Project>>);
