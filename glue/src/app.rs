pub use editor::Editor;
pub use home::Home;
pub use querying_backend::QueryingBackend;
pub use setup::Setup;

mod editor;
mod home;
mod querying_backend;
mod setup;
use chipbox_common as common;
use std::rc::Rc;

#[cfg(feature = "backend")]
use chipbox_backend_lib as backend_lib;
#[cfg(feature = "backend")]
use std::time::Duration;

pub trait ConfiguredState {
    fn settings(&self) -> Rc<common::Settings>;
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum App {
    QueryingBackend(QueryingBackend),
    Setup(Setup),
    Home(Home),
    Editor(Editor),
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
            backend_lib::App::ProjectSelection(project_selection) => {
                Self::Home(project_selection.into())
            }
            backend_lib::App::Editor(editor) => {
                Self::Editor(editor.as_ref().into())
            }
        }
    }
}

#[cfg(feature = "backend")]
const TIMEOUT: Duration = Duration::from_secs(10);

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq,
)]
pub struct StateTimedOutError;

impl std::error::Error for StateTimedOutError {}
impl std::fmt::Display for StateTimedOutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "timed out while waiting for state")
    }
}

#[cfg(feature = "backend")]
#[tauri::command]
pub(crate) async fn app<R>(
    app_handle: tauri::AppHandle<R>,
) -> Result<App, StateTimedOutError>
where
    R: tauri::Runtime,
{
    use tauri::Manager as _;
    use tokio::time::sleep;

    // Query `backend_lib::App` with a delay of 50ns on failure.
    let begin = std::time::Instant::now();
    let state = loop {
        match app_handle.try_state::<backend_lib::ManagedApp>() {
            Some(state) => break state,
            None => {
                let elapsed = begin.elapsed();
                if elapsed >= TIMEOUT {
                    return Err(StateTimedOutError);
                } else {
                    sleep(Duration::from_nanos(50)).await;
                    tracing::info!("waiting for `backend_lib::App`...");
                }
            }
        }
    };

    // Lock mutex and convert `backend_lib::App` to `glue::App`.
    let backend_app = state.arc.lock().await;
    Ok((&*backend_app).into())
}

#[cfg(feature = "frontend")]
pub async fn query() -> Result<App, StateTimedOutError> {
    use crate::invoke::*;
    invoke_query::<App, StateTimedOutError, ()>("app", &()).await
}
