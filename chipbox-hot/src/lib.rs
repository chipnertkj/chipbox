//! # Note
//! This crate provides hot-reloadable functions for crate `chipbox`.
//!
//! Since this library is compiled into a `dylib`, all public exports must be
//! non-generic functions with `#[no_mangle]`.
//!
//! There's a more exhaustive list of limitations [here](https://github.com/rksm/hot-lib-reloader-rs?tab=readme-ov-file#know-the-limitations).
//!
//! Failure to follow may result in undefined behavior and/or crashes.

use no_mangle_if_debug::no_mangle_if_debug;

#[no_mangle_if_debug]
pub fn abc() {}
