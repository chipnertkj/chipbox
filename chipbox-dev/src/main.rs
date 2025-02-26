//! [`chipbox`](https://github.com/chipnertkj/chipbox) development tool.

use std::path::{Path, PathBuf};

use chipbox_dev::Verbosity;
use clap::Parser as _;
use miette::{Context as _, IntoDiagnostic as _};
use relative_path::RelativePath;
use tokio::fs;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = chipbox_dev::Args::parse();
    init_tracing(&args)?;
    let cwd = working_dir()?;
    let config_path = config_path(&cwd);
    let config_string = read_config_file(&config_path).await?;
    let config = parse_config(&config_string)?;
    validate_config(&config, &config_path)?;
    Ok(())
}

/// Get the config path based on provided working directory.
fn config_path(cwd: &Path) -> PathBuf {
    let config_path = RelativePath::new("chipbox-dev.toml").to_path(cwd);
    config_path
}

/// Get the current working directory.
fn working_dir() -> miette::Result<PathBuf> {
    let cwd = std::env::current_dir()
        .into_diagnostic()
        .wrap_err("get current working directory")?;
    debug_display_working_dir(&cwd)?;
    Ok(cwd)
}

/// Validate provided config.
/// Path parameter is used for error reporting.
fn validate_config(config: &chipbox_dev::Config, path: &Path) -> miette::Result<()> {
    config
        .validate()
        .wrap_err_with(|| format!("path: {}", path.display()))
        .wrap_err("validate config")?;
    tracing::debug!("config validated");
    tracing::info!("using config from working dir: {}", path.display());
    Ok(())
}

/// Parse provided config string to [`chipbox_dev::Config`].
fn parse_config(string: &str) -> miette::Result<chipbox_dev::Config> {
    let config = toml::from_str(string)
        .into_diagnostic()
        .wrap_err("parse config")?;
    tracing::trace!("parsed config: {:#?}", config);
    Ok(config)
}

/// Read config file at provided path to a string.
async fn read_config_file(path: &Path) -> miette::Result<String> {
    fs::read_to_string(path)
        .await
        .into_diagnostic()
        .wrap_err_with(|| format!("path: {}", path.display()))
        .wrap_err("read config file")
}

/// Display the current working directory to output.
/// Only reports on verbosity level `debug` or higher.
fn debug_display_working_dir(path: &Path) -> miette::Result<()> {
    tracing::debug!("working dir: {}", path.display());
    Ok(())
}

/// Get the default verbosity level for the application.
/// The value depends on whether debug assertions are enabled.
fn default_verbosity() -> Verbosity {
    cfg!(debug_assertions)
        .then(|| Verbosity::Debug {
            from_debug_assertions: true,
        })
        .unwrap_or(Verbosity::Normal)
}

/// Initialize tracing capabilities based on provided arguments.
fn init_tracing(args: &chipbox_dev::Args) -> miette::Result<()> {
    let verbosity = args.verbosity().max(default_verbosity());
    let directives = tracing_directives(verbosity);
    chipbox_utils::tracing::init_subscriber(directives).wrap_err("init subscriber")?;
    // Emit message if verbosity isn't normal.
    debug_display_verbosity(verbosity);
    Ok(())
}

/// Display the verbosity level to output.
/// Only reports on verbosity level `debug` or higher.
fn debug_display_verbosity(verbosity: Verbosity) {
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
}

/// Generate tracing directives based on verbosity level.
fn tracing_directives(verbosity: Verbosity) -> Vec<String> {
    let crate_name = env!("CARGO_CRATE_NAME");
    match verbosity {
        Verbosity::Normal => vec![format!("{crate_name}=info")],
        Verbosity::Debug { .. } => vec![format!("{crate_name}=debug")],
        Verbosity::Trace => vec![format!("{crate_name}=trace")],
    }
}
