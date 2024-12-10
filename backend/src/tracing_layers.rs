//! Functionality for initializing tracing capabilities.

use crate::paths;
use color_eyre::eyre::{Context, ContextCompat};
use tauri::async_runtime;
use tracing::subscriber;
use tracing_appender::{
    non_blocking::{NonBlocking, WorkerGuard},
    rolling,
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, Layer, Registry};

fn stdout_layer() -> impl Layer<Registry> + 'static {
    fmt::layer().with_writer(std::io::stdout)
}

/// Initialize and set a global tracing subscriber with a file logger.
///
/// Returns a worker guard. Dropping it will close the logger.
pub fn init() -> color_eyre::Result<WorkerGuard> {
    // Construct subscriber.
    let (appender, guard) = appender().wrap_err("failed to create appender")?;
    let file_log_layer = fmt::layer().with_writer(appender).with_ansi(false);
    let subscriber = Registry::default()
        .with(stdout_layer())
        .with(file_log_layer);
    // Install subscriber.
    subscriber::set_global_default(subscriber)?;
    Ok(guard)
}

/// Create a [`tracing_appender`] file writer.
///
/// The file used by the writer is chosen by [`paths::log::file_path`].
fn appender() -> color_eyre::Result<(NonBlocking, WorkerGuard)> {
    // Get log file path.
    let rt = async_runtime::handle();
    let file_path = rt.block_on(paths::log::file_path())?;
    let parent_path = file_path
        .parent()
        .wrap_err("unable to get parent path of log file")?;
    let file_name = file_path
        .file_name()
        .wrap_err("unable to get file name of log file")?;
    // Create log file appender.
    let appender = rolling::never(parent_path, file_name);
    Ok(tracing_appender::non_blocking(appender))
}
