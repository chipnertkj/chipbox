//! [`chipbox`](https://github.com/chipnertkj/chipbox) development tool.

use chipbox_dev::Verbosity;
use clap::Parser as _;
use miette::{Context as _, IntoDiagnostic as _};
use std::path::Path;
use tokio::fs;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = chipbox_dev::Args::parse();
    init_tracing(args.verbosity())?;
    debug_display_working_dir()?;
    let config_path = Path::new("chipbox-dev.toml");
    let config_string = read_config_file_string(config_path).await?;
    let config = parse_config(&config_string)?;
    validate_config(&config, config_path)?;
    display_config_path_ok(config_path)?;
    Ok(())
}

/// Display a message confirming the config file at `path` is being used.
fn display_config_path_ok(path: &Path) -> miette::Result<()> {
    let path_str = chipbox_utils::path::display_limit_ancestors(path, 3)?;
    tracing::info!("using config from working dir: {}", path_str);
    Ok(())
}

/// Validate provided config.
/// Path parameter is used for error reporting.
fn validate_config(config: &chipbox_dev::Config, path: &Path) -> miette::Result<()> {
    config
        .validate()
        .wrap_err_with(|| format!("validate config: {}", path.display()))?;
    tracing::debug!("config validated");
    Ok(())
}

/// Parse provided config string.
fn parse_config(string: &str) -> miette::Result<chipbox_dev::Config> {
    let config = toml::from_str(string)
        .into_diagnostic()
        .wrap_err("parse config")?;
    tracing::trace!("parsed config: {:#?}", config);
    Ok(config)
}

/// Read config file at provided path to a string.
async fn read_config_file_string(path: &Path) -> miette::Result<String> {
    fs::read_to_string(path)
        .await
        .into_diagnostic()
        .wrap_err("read config file")
}

/// Display the current working directory to output.
/// Debug verbosity message.
fn debug_display_working_dir() -> miette::Result<()> {
    let working_dir = std::env::current_dir()
        .into_diagnostic()
        .wrap_err("get working dir")?;
    tracing::debug!("working dir: {}", working_dir.display());
    Ok(())
}

/// Initialize tracing capabilities with verbosity level.
fn init_tracing(verbosity: Verbosity) -> miette::Result<()> {
    let default_verbosity = cfg!(debug_assertions)
        .then(|| Verbosity::Debug {
            from_debug_assertions: true,
        })
        .unwrap_or(Verbosity::Normal);
    let verbosity = verbosity.max(default_verbosity);
    let directives = directives(verbosity);
    chipbox_utils::tracing::init_subscriber(directives).wrap_err("init subscriber")?;
    // Emit message if verbosity isn't normal.
    match verbosity {
        // Skip on normal.
        Verbosity::Normal => {}
        // Mention debug assertions if they are the reason for the verbosity level.
        Verbosity::Debug {
            from_debug_assertions: true,
        } => {
            tracing::debug!("running in verbose ({verbosity}) mode due to debug assertions");
        }
        // Otherwise just report level.
        _ => {
            tracing::debug!("running in verbose ({verbosity}) mode");
        }
    }
    Ok(())
}

/// Generate tracing directives based on verbosity level.
fn directives(verbosity: Verbosity) -> Vec<String> {
    let crate_name = env!("CARGO_CRATE_NAME");
    match verbosity {
        Verbosity::Normal => vec![],
        Verbosity::Debug { .. } => vec![format!("{crate_name}=debug")],
        Verbosity::Trace => vec![format!("{crate_name}=trace")],
    }
}
