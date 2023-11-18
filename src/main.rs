use chipbox_lib as lib;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

mod ui;

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
    yew::Renderer::<ui::App>::new().render();
}
