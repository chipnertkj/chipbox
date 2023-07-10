//! This crate defines data structures common between crates `chipbox` and `chipbox-core`.
//! ## Note for developers
//! Since this crate may be compiled to wasm, take care not to use system APIs.
//! Ideally, you should define system dependent logic in `chipbox-core`,
//! for example by taking advantage of the newtype pattern.

mod app_state;

pub use app_state::*;
