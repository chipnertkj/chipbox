#![feature(never_type)]
#![feature(try_find)]
#![feature(iter_repeat_n)]
// Enable Windows application subsystem on release.
// This prevents an additional console window from showing up on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Lints disabled for development purposes:
#![allow(dead_code)]

pub mod path; // ok
pub mod state; // ok

pub use state::State;
