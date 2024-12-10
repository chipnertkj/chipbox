//! This library implements the backend part of `chipbox`.
//!
//! It boots up a [`tauri`] application and handles requests from the frontend.
//! It also manages most system resources required to run the application,
//! such as audio devices and file handles, used to render audio as part of the
//! functionality provided by the whole application.

mod file;
mod paths;
mod tauri_run;
mod tracing_layers;

pub use tauri_run::run;
