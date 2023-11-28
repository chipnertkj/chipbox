// Required by `const_format::formatc!`.
#![feature(const_mut_refs)]
// Required by `chipbox_ui_panel::Panel`.
#![feature(generic_const_exprs)]

pub use app::App;
pub mod app;
