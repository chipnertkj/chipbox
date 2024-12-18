use miette::{Context as _, IntoDiagnostic as _};
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

pub(super) fn init(verbose: bool) -> miette::Result<()> {
    let directive_level = if verbose { "trace" } else { "info" };
    let crate_name = env!("CARGO_CRATE_NAME");
    let trace_directive = format!("{crate_name}={directive_level}")
        .parse()
        .into_diagnostic()
        .wrap_err("error parsing tracing filter")?;
    let env_filter = EnvFilter::from_default_env().add_directive(trace_directive);

    SubscriberBuilder::default()
        .with_env_filter(env_filter)
        .without_time()
        .try_init()
        .map_err(|e| miette::miette!("error initializing tracing subscriber: {e}"))?;
    if verbose {
        tracing::debug!("running in verbose mode");
    }

    Ok(())
}
