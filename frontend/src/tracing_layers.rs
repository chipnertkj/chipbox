//! Functionality for initializing tracing capabilities.

use tracing::subscriber;
use tracing_subscriber::{
    fmt::{self, format::Pretty},
    layer::SubscriberExt as _,
    EnvFilter, Layer, Registry,
};
use tracing_web::{performance_layer, MakeWebConsoleWriter};

/// Initialize and set a global tracing subscriber.
pub(super) fn init() {
    // Construct performance layer.
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    // Construct env filter.
    let env_filter =
        EnvFilter::from_default_env().add_directive("debug".parse().expect("invalid directive"));
    // Construct subscriber.
    let subscriber = {
        tracing_subscriber::registry()
            .with(fmt_layer())
            .with(perf_layer)
            .with(env_filter)
    };
    // Install subscriber.
    subscriber::set_global_default(subscriber).expect("unable to set global default subscriber");
}

/// Create a [`tracing_web`] console writer.
///
/// This adds a writer that outputs the trace to the web console.
fn console_writer() -> MakeWebConsoleWriter {
    MakeWebConsoleWriter::new().with_pretty_level()
}

/// Construct a pretty-formatted tracing layer for console output.
fn fmt_layer() -> impl Layer<Registry> + 'static {
    fmt::layer()
        // File, line, and target are always included.
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        // `std::time` is not available in browsers.
        .without_time()
        // Add console writer.
        .with_writer(console_writer())
        .with_level(false)
}
