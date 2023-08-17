#![feature(never_type)]

#[cfg(feature = "backend")]
pub mod handler; // ok
#[cfg(feature = "frontend")]
mod invoke; // ok

pub mod app;
