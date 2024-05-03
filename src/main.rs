mod tracing_setup;
mod ui_setup;

/// Application entry point.
fn main() {
    tracing_setup::init();
    ui_setup::init();
}
