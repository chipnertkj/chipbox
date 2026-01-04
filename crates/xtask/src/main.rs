//! Automation utilities for [`chipbox`](../chipbox).
//! See <https://github.com/matklad/cargo-xtask> for context.
//!
//! Run `cargo xtask` for usage information.

#![feature(iter_intersperse)]

use clap::Parser as _;
use miette::Context as _;

mod app;
mod build;
mod command;
mod dev;
mod fs;
mod logger;
mod portaudio;
mod program;
mod tui;

#[derive(clap::Parser)]
#[command(about)]
enum Args {
    /// Start the development server.
    Dev,
    /// Build the final application.
    Build,
}

/// Execute commands.
#[tokio::main]
async fn main() -> miette::Result<()> {
    let program = match Args::parse() {
        Args::Dev => dev::program().await.wrap_err("dev"),
        Args::Build => build::program().await.wrap_err("build"),
    }
    .wrap_err("prepare xtask program")?;
    todo!("run program");
}
