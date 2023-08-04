//! This module provides the necessary functionality for
//! interop between the backend and frontend applications.

#![feature(never_type)]
#![feature(if_let_guard)]

pub use chipbox_backend_lib::state::Settings;
pub use chipbox_backend_lib::State;

#[cfg(feature = "backend")]
pub mod handler; // ok
#[cfg(feature = "frontend")]
pub(crate) mod invoke; // ok

// commands:
pub mod state;
