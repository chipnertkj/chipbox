use miette::{Context as _, IntoDiagnostic as _, miette};
use tracing_subscriber::{EnvFilter, fmt::SubscriberBuilder};

/// Initialize tracing capabilities with [`tracing_subscriber`].
/// Subscriber is configured not to include timestamps.
///
/// # Arguments
/// - `directives` - list of directives to apply to the [`EnvFilter`].
pub fn init_subscriber(
    directives: impl IntoIterator<Item = impl AsRef<str>>,
) -> miette::Result<()> {
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
        // Error is type-erased, need to map to valid diagnostic.
        .map_err(|e| miette!("{}", e.to_string()))
        .wrap_err("install")?;
    Ok(())
}
