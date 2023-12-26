use crate::common;
use common::app::FrontendMsg;

#[cfg(feature = "backend")]
use {crate::backend_lib::ThreadMsg, tauri::async_runtime::Sender};

/// Wrapper around a `async_runtime::Sender` for the backend lib app thread.
/// Used as a managed state in the tauri app.
#[cfg(feature = "backend")]
pub struct BackendLibTx(pub Sender<ThreadMsg>);

#[cfg(feature = "backend")]
impl BackendLibTx {
    fn inner_tx_from_app<R>(app: &tauri::AppHandle<R>) -> &Sender<ThreadMsg>
    where
        R: tauri::Runtime,
    {
        use tauri::Manager as _;
        let BackendLibTx(tx) = app
            .state::<BackendLibTx>()
            .inner();
        tx
    }
}

#[cfg(feature = "backend")]
#[tauri::command]
pub(crate) async fn frontend_msg<R>(
    app: tauri::AppHandle<R>,
    msg: FrontendMsg,
) -> bool
where
    R: tauri::Runtime,
{
    let res = BackendLibTx::inner_tx_from_app(&app)
        .send(ThreadMsg::Frontend(msg))
        .await;
    if res.is_err() {
        tracing::error!(
            "Main -> app thread message failed. Has it panicked? Result: {:?}",
            res
        );
    }
    res.is_ok()
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
