//! Desktop DAW built around a visual node graph synthesizer.

use miette::Context as _;

fn main() -> miette::Result<()> {
    let directives = tracing_directives();
    chipbox_utils::tracing::init_subscriber(directives).wrap_err("init subscriber")?;
    let mut app = chipbox::App::new();
    app.run()
}

/// Generate tracing directives.
fn tracing_directives() -> impl IntoIterator<Item = String> {
    let crate_name = env!("CARGO_CRATE_NAME");
    let trace_crates = [crate_name, "chipnertkj_ui_render"];
    trace_crates.map(|c| format!("{c}=trace"))
}
