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
    print_working_dir()?;
    let config_path = Path::new("chipbox-dev.toml");
    let config_string = read_config_file(config_path).await?;
    let config = parse_config(&config_string)?;
    validate_config(&config, config_path)?;
    Ok(())
}

/// Validate provided config.
/// Path parameter is used for error reporting.
fn validate_config(config: &chipbox_dev::Config, config_path: &Path) -> miette::Result<()> {
    config
        .validate()
        .wrap_err_with(|| format!("validate config: {}", config_path.display()))?;
    tracing::debug!("config validated");
    Ok(())
}

/// Parse provided config string.
fn parse_config(config_string: &str) -> miette::Result<chipbox_dev::Config> {
    let config = toml::from_str(config_string)
        .into_diagnostic()
        .wrap_err("parse config")?;
    tracing::trace!("parsed config: {:#?}", config);
    Ok(config)
}

/// Read config file at provided path.
async fn read_config_file(path: &Path) -> miette::Result<String> {
    fs::read_to_string(path)
        .await
        .into_diagnostic()
        .wrap_err("read config file")
}

/// Print current working directory.
fn print_working_dir() -> miette::Result<()> {
    let working_dir = std::env::current_dir()
        .into_diagnostic()
        .wrap_err("get working dir")?;
    tracing::debug!("working dir: {}", working_dir.display());
    Ok(())
}

/// Initialize tracing capabilities with verbosity level.
fn init_tracing(verbosity: Verbosity) -> miette::Result<()> {
    let default_verbosity = cfg!(debug_assertions)
        .then(|| Verbosity::Debug)
        .unwrap_or(Verbosity::Normal);
    let verbosity = verbosity.max(default_verbosity);
    let directives = directives(verbosity);
    chipbox_utils::init_tracing(directives).wrap_err("init tracing")?;
    if verbosity > Verbosity::Normal {
        tracing::debug!("running in verbose ({}) mode", verbosity);
    }
    Ok(())
}

/// Generate tracing directives based on verbosity level.
fn directives(verbosity: Verbosity) -> Vec<String> {
    let crate_name = env!("CARGO_CRATE_NAME");
    match verbosity {
        Verbosity::Normal => vec![],
        Verbosity::Debug => vec![format!("{crate_name}=debug")],
        Verbosity::Trace => vec![format!("{crate_name}=trace")],
    }
}
