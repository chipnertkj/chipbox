use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

/// Initialize `tracing`, `tracing_subscriber` and `tracing_web`.
pub(super) fn init() {
    let fmt_layer = move || {
        tracing_subscriber::fmt::layer()
            // WebViews do not support time.
            .without_time()
            .with_writer(tracing_web::MakeConsoleWriter)
    };

    let performance_layer = move || {
        tracing_web::performance_layer()
            .with_details_from_fields(Pretty::default())
    };

    tracing_subscriber::registry()
        .with(fmt_layer())
        .with(performance_layer())
        .init();
}
