// Required by `Panel`.
#![feature(generic_const_exprs)]
// Disables warn from `generic_const_exprs`.
#![allow(incomplete_features)]

mod panel;
mod tab;

pub use panel::Panel;
pub use tab::Tab;
