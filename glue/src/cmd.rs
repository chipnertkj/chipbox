//! Commands callable from the frontend.
//! TODO: module comment too vague

pub mod create_project;
pub mod loaded_project;

#[cfg(feature = "frontend")]
pub use {create_project::frontend::create_project, loaded_project::frontend::loaded_project};
