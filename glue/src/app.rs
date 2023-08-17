pub use querying_backend::QueryingBackend;
pub use setup::Setup;

mod querying_backend;
mod setup;
#[cfg(feature = "backend")]
use chipbox_backend_lib as backend_lib;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum App {
    QueryingBackend(QueryingBackend),
    Setup(Setup),
    Home,
    Editor,
}

impl Default for App {
    fn default() -> Self {
        App::QueryingBackend(QueryingBackend::default())
    }
}

#[cfg(feature = "backend")]
impl From<&backend_lib::App> for App {
    fn from(backend_app: &backend_lib::App) -> Self {
        match backend_app {
            backend_lib::App::ReadingSettings => {
                Self::QueryingBackend(QueryingBackend::ReadingSettings)
            }
            backend_lib::App::Setup(setup) => Self::Setup(setup.into()),
            backend_lib::App::ProjectSelection(_) => Self::Home,
            backend_lib::App::Editor => Self::Editor,
        }
    }
}

#[cfg(feature = "backend")]
#[tauri::command]
pub(crate) async fn app<R>(app_handle: tauri::AppHandle<R>) -> App
where
    R: tauri::Runtime,
{
    use std::sync::Arc;
    use std::time::Duration;
    use tauri::Manager as _;
    use tokio::sync::Mutex;
    use tokio::time::sleep;

    /// As managed by `tauri::App` in `chipbox_backend`.
    /// Note that we use `tokio::sync::Mutex`.
    type ManagedApp = Arc<Mutex<backend_lib::App>>;

    // Query `backend_lib::App` with a delay of 50ns on failure.
    let state = loop {
        match app_handle.try_state::<ManagedApp>() {
            Some(state) => break state,
            None => {
                sleep(Duration::from_nanos(50)).await;
                tracing::info!("waiting for `backend_lib::App`...");
            }
        }
    };

    // Lock mutex and convert `backend_lib::App` to `glue::App`.
    let backend_app = state.lock().await;
    (&*backend_app).into()
}

#[cfg(feature = "frontend")]
pub async fn query() -> App {
    use crate::invoke::*;
    invoke_query_infallible::<App, ()>("app", &()).await
}
