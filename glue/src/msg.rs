use crate::common;
use common::app::FrontendMsg;

#[cfg(feature = "backend")]
use {crate::backend_lib::ThreadMsg, tauri::async_runtime::Sender};

/// Wrapper around a `async_runtime::Sender` for the backend lib app thread.
/// Used as a managed state in the tauri app.
#[cfg(feature = "backend")]
pub struct BackendAppTx(pub Sender<ThreadMsg>);

#[cfg(feature = "backend")]
impl BackendAppTx {
    fn inner_tx_from_app<R>(app: &tauri::AppHandle<R>) -> &Sender<ThreadMsg>
    where
        R: tauri::Runtime,
    {
        use tauri::Manager as _;
        let BackendAppTx(tx) = app
            .state::<BackendAppTx>()
            .inner();
        tx
    }
}

/// Send frontend message to backend app thread.
#[cfg(feature = "backend")]
#[tauri::command]
pub(crate) async fn frontend_msg<R>(
    app: tauri::AppHandle<R>,
    msg: FrontendMsg,
) -> bool
where
    R: tauri::Runtime,
{
    use tokio::sync::mpsc::error::SendError;

    tracing::trace!("Forwarding frontend message to backend thread: {:?}", msg);

    // Send frontend message to backend thread.
    let result = BackendAppTx::inner_tx_from_app(&app)
        .send(ThreadMsg::Frontend(msg))
        .await;

    // Handle result.
    match result {
        Ok(()) => true,
        Err(SendError(msg)) => {
            // Fail gracefully.
            tracing::error!(
                "Failed to deliver frontend message to backend. Channel was closed. Original message: {msg:?}",
            );
            false
        }
    }
}

#[cfg(feature = "frontend")]
#[must_use]
pub async fn send(msg: FrontendMsg) -> bool {
    #[derive(serde::Serialize, Debug)]
    struct FrontendMsgCmdArgs {
        msg: FrontendMsg,
    }

    crate::invoke::invoke_query_infallible::<bool, FrontendMsgCmdArgs>(
        "frontend_msg",
        &FrontendMsgCmdArgs { msg },
    )
    .await
}
