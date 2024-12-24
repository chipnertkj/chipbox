#![feature(never_type)]

pub mod block;
pub mod frame;
pub mod synth;

pub use synth::{cmd, Graph, GraphError};
