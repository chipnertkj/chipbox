// Required by `const_format::formatc!`.
#![feature(const_mut_refs)]
// Required by `chipbox_ui_panel::Panel`.
#![feature(generic_const_exprs)]
// Disables warn from `generic_const_exprs`.
#![allow(incomplete_features)]

pub use app::App;
pub mod app;
