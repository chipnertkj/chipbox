use crate::backend_lib::{AppThread, ThreadMsg};
use crate::glue::msg::BackendAppTx;
use tauri::async_runtime::{self, JoinHandle, Mutex};
use tauri::Manager as _;

/// Thread handle for the app thread.
#[derive(Default)]
pub(super) struct ManagedJoinHandle(Mutex<Option<JoinHandle<()>>>);

/// Join the app thread.
async fn join(managed_app_thread_handle: &mut ManagedJoinHandle) {
    tracing::trace!("Waiting for app thread to finish.");

    // Lock app thread join handle mutex.
    let ManagedJoinHandle(ref mut mutex) = managed_app_thread_handle;
    let mut lock = mutex.lock().await;

    // Take ownership of the join handle.
    match lock.take() {
        // Thread is running as expected.
        Some(join_handle) => {
            // Join the app thread.
            join_handle
                .await
                // Fail gracefully.
                .unwrap_or_else(|err| {
                    // Join failed.
                    tracing::error!(
                        "Failed to join app thread due to panic: {err}"
                    );
                });
        }
        // Thread was aleady closed (or ownership was lost).
        None => {
            // Fail gracefully.
            // App thread was closed or closing it is out of scope.
            tracing::error!(
                "Attempted to join app thread while it was not running."
            );
        }
    }

    // All ok.
    tracing::trace!("App thread joined.");
}

/// Start the backend-lib app thread
/// and set up a message channel between them.
pub async fn start(
    managed_app_thread_handle: &mut ManagedJoinHandle,
    app: &tauri::AppHandle,
) {
    // Create message channels.
    let (tx, rx) = async_runtime::channel::<ThreadMsg>(128);

    // Store the message TX in the app state.
    app.manage(BackendAppTx(tx));

    // Start the app thread.
    let join_handle =
        async_runtime::spawn(AppThread::new(rx, app.clone()).run());

    // Store the join handle.
    let ManagedJoinHandle(ref mut mutex) = managed_app_thread_handle;
    *mutex.lock().await = Some(join_handle);
}

/// Send an exit message to the backend-lib app thread
/// and wait for it to finish.
pub async fn close(
    managed_app_thread_handle: &mut ManagedJoinHandle,
    app: &tauri::AppHandle,
) {
    tracing::trace!("Sending exit message to app thread.");

    // Get message sender.
    match app.try_state::<BackendAppTx>() {
        // Message sender state is available.
        Some(state) => {
            // Get message sender.
            let BackendAppTx(ref tx) = state.inner();

            // Send exit message.
            match tx.send(ThreadMsg::Exit).await {
                // Message sent.
                Ok(_) => {
                    // Join the app thread.
                    join(managed_app_thread_handle).await;
                }
                // The channel was already closed.
                Err(_) => {
                    // Fail gracefully.
                    // Channel is closed, so we can assume that the app thread has finished.
                    tracing::error!("Sending exit message to app thread failed, channel was already closed.");
                }
            }
        }
        // Could not get message sender from app state.
        None => {
            // Fail gracefully.
            // App state is not available, so we can assume that the app thread was never started.
            tracing::error!("Sending exit message to app thread failed, app thread TX was not stored in app state.");
        }
    };

    // All ok.
    tracing::trace!("App closed.");
}
