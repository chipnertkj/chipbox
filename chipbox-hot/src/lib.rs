//! # Note
//! **All** public exports must:
//! - Be `#[no_mangle]`.
//! - Keep the same function signature between reloads.
//!
//! Failure to follow may result in undefined behavior and/or crashes.
//! This only applies to builds with the `hot` feature enabled.

#[unsafe(no_mangle)]
pub fn abc() {}
