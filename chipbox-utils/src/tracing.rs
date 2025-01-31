use miette::{Context as _, IntoDiagnostic as _};
use tracing_subscriber::{EnvFilter, fmt::SubscriberBuilder};

/// Initialize tracing capabilities.
///
/// # Arguments
/// * `directives` - list of directives to add to the env filter.
pub fn init(directives: impl IntoIterator<Item = impl AsRef<str>>) -> miette::Result<()> {
    // Generate env filter.
    let mut env_filter = EnvFilter::from_default_env();
    let directives = directives
        .into_iter()
        .map(|s| s.as_ref().parse())
        .collect::<Result<Vec<_>, _>>()
        .into_diagnostic()
        .wrap_err("parse directive")?;
    // Apply directives to env filter.
    for directive in directives {
        env_filter = env_filter.add_directive(directive);
    }
    // Build subscriber.
    SubscriberBuilder::default()
        .with_env_filter(env_filter)
        .without_time()
        .try_init()
        .map_err(|e| miette::miette!("init subscriber: {e}"))?;
    Ok(())
}
