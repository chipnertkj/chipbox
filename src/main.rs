//! Chipbox `yew` application, referred to as the frontend.
//!
//! This crate is responsible for:
//! - setting up `yew`,
//! - providing commands for backend communication,
//! - implementing the user interface for managing the backend state.
//!
//! # Inline comments for external file modules
//! For clarity, every finished external file module should have an inline comment
//! right next to it, containing "ok".

#![feature(never_type)]

mod components;
mod util; // ok

use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

/// Initialize `console_error_panic_hook` on debug.
/// This forwards Rust panic traces to the console.
fn setup_panic_console_hook() {
    if cfg!(debug_assertions) {
        console_error_panic_hook::set_once();
    }
}

/// Initialize `tracing`, `tracing_subscriber` and `tracing_web`.
fn setup_tracing() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        // WebViews do not support time.
        .without_time()
        .with_writer(tracing_web::MakeConsoleWriter);
    let performance_layer = tracing_web::performance_layer()
        .with_details_from_fields(
            tracing_subscriber::fmt::format::Pretty::default(),
        );
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(performance_layer)
        .init();
}

/// Application entry point.
fn main() {
    setup_panic_console_hook();
    setup_tracing();
    yew::Renderer::<components::App>::new().render();
}
