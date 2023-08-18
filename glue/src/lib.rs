#![feature(never_type)]

#[cfg(feature = "backend")]
pub mod handler;
#[cfg(feature = "frontend")]
mod invoke; // ok // ok

pub mod app;
pub use app::App;
