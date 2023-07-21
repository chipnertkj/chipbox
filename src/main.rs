#![feature(never_type)]
// Lints disabled for development purposes:
#![allow(dead_code)]

mod commands;
mod components;

use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

/// `console_error_panic_hook` forwards Rust panic traces to the console.
fn panic_console_hook() {
    if cfg!(debug_assertions) {
        console_error_panic_hook::set_once();
    }
}

fn tracing_subscriber_init() {
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

fn main() {
    panic_console_hook();
    tracing_subscriber_init();
    yew::Renderer::<components::App>::new().render();
}
