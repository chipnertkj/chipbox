use crate::backend_lib::{AppThread, ThreadMsg};
use crate::glue::msg::BackendLibTx;
use tauri::async_runtime::{self, JoinHandle, Mutex};
use tauri::Manager as _;

/// Thread handle for the backend lib app thread.
type ManagedJoinHandle = Mutex<Option<JoinHandle<()>>>;

/// Static mutex with a join handle for the backend lib app thread.
///
/// # Expected behavior
/// In order.
/// - Starts as `None`.
/// - Set to `Some` when `app_thread::start` is called.
/// - Set back to `None` when `app_thread::join` is called.
/// See: `Option<T>::take`.
static BACKEND_LIB_THREAD: once_cell::sync::Lazy<ManagedJoinHandle> =
    once_cell::sync::Lazy::new(|| Mutex::new(None));

/// Join the backend lib app thread.
async fn join() {
    tracing::trace!("Waiting for `join_app_thread` to finish.");
    // Lock backend lib app thread join handle mutex.
    let mut lock = BACKEND_LIB_THREAD
        .lock()
        .await;
    // Acquire the join handle.
    let join_handle = lock
        .take()
        .expect("`join_app_thread` called before `start_app_thread`");
    // Join the app thread.
    join_handle
        .await
        .expect("`join_app_thread` failed");
}

/// Start the backend lib app thread and set up message TX + RX.
pub async fn start(app: &tauri::AppHandle) {
    // Create message channels.
    let (tx, rx) = async_runtime::channel::<ThreadMsg>(128);
    // Store the message TX in the app state.
    app.manage(BackendLibTx(tx));
    // Start the app thread.
    let join_handle =
        async_runtime::spawn(AppThread::new(rx, app.clone()).run());
    // Store the join handle in the static mutex.
    *BACKEND_LIB_THREAD
        .lock()
        .await = Some(join_handle);
}

/// Send an exit message to the backend lib app thread
/// and wait for it to finish.
pub async fn close(app: &tauri::AppHandle) {
    tracing::trace!("Sending exit message to backend lib app thread.");
    // Get message sender.
    let BackendLibTx(tx) = app
        .state::<BackendLibTx>()
        .inner();
    // Send exit message.
    tx.send(ThreadMsg::Exit)
        .await
        .expect("`join_app_thread` send failed, rx was closed");
    // Join the app thread.
    join().await;
}
