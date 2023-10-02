#![feature(never_type)]

#[cfg(feature = "backend")]
pub mod handler;
#[cfg(feature = "frontend")]
mod invoke; // ok // ok

#[cfg(feature = "backend")]
type ManagedApp = std::sync::Arc<tokio::sync::Mutex<chipbox_backend_lib::App>>;

// Commands.
pub mod app;
pub mod load_project;
pub mod skip_setup;

pub use app::App;
pub use load_project::LoadProjectInfo;
