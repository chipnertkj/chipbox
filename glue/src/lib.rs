#![feature(never_type)]

#[cfg(feature = "backend")]
pub mod handler;
#[cfg(feature = "frontend")]
mod invoke; // ok // ok

#[cfg(feature = "backend")]
type ManagedApp = std::sync::Arc<tokio::sync::Mutex<chipbox_backend_lib::App>>;

pub mod app;
pub use app::App;

pub mod skip_setup;
