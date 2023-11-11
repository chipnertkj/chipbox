#![feature(never_type)]

#[cfg(feature = "backend")]
pub mod handler;
#[cfg(feature = "frontend")]
mod invoke;

// Commands.
pub mod app;
pub mod load_project;
pub mod set_settings;
pub mod skip_setup;

pub use app::{App, ConfiguredState, Setup};
pub use load_project::LoadProjectInfo;
