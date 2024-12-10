//! Glue code used for communication between the frontend and backend.

#![warn(
    missing_docs,
    unreachable_pub,
    missing_debug_implementations,
    rust_2018_idioms
)]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(never_type)]

#[cfg(feature = "backend")]
pub mod handler;
#[cfg(feature = "backend")]
pub mod loaded_project;
#[cfg(feature = "frontend")]
pub(crate) mod tauri_api;

pub mod cmd;
