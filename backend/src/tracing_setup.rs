use crate::backend_lib;
use backend_lib::dir;
use color_eyre::eyre;
use tracing::subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt as _;

/// Initialize tracing and the file logger.
///
/// Returns a guard. Dropping it will close the file logger.
pub(super) fn init(
    rt: tauri::async_runtime::RuntimeHandle,
) -> eyre::Result<WorkerGuard> {
    // Construct data path.
    let data_path = rt.block_on(async move { dir::data_path().await })?;

    /// Log file name.
    const LOG_FILE_NAME: &str = "chipbox-backend.log";
    // Create log file.
    std::fs::File::create(data_path.join(LOG_FILE_NAME))?;

    // Create log file appender.
    let appender = rolling::never(data_path, LOG_FILE_NAME);
    let (non_blocking_appender, guard) =
        tracing_appender::non_blocking(appender);

    // Set up std-out logging.
    let stdout_layer = fmt::layer().with_writer(std::io::stdout);

    // Set up file logging
    let file_log_layer = fmt::layer()
        .with_writer(non_blocking_appender)
        .with_ansi(false);

    // Construct subscriber.
    let subscriber = tracing_subscriber::Registry::default()
        .with(stdout_layer)
        .with(file_log_layer);

    // Install subscriber.
    subscriber::set_global_default(subscriber)?;

    Ok(guard)
}
