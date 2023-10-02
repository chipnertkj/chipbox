#[cfg(feature = "backend")]
use chipbox_backend_lib as backend_lib;
use chipbox_common::project_meta::ProjectPath;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum Error {
    NotApplicable,
    IO,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`load_project` is N/A in this context")
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum LoadProjectInfo {
    New,
    Load(ProjectPath),
}

#[cfg(feature = "backend")]
#[tauri::command]
pub(crate) async fn load_project(
    backend_app: tauri::State<super::ManagedApp, '_>,
    info: LoadProjectInfo,
) -> Result<(), Error> {
    let mut backend_app = backend_app.lock().await;
    match &mut *backend_app {
        backend_lib::App::ProjectSelection(project_selection) => match info {
            LoadProjectInfo::New => {
                *backend_app = backend_lib::App::Editor;
                Ok(())
            }
            LoadProjectInfo::Load(path) => {
                *backend_app = backend_lib::App::Editor;
                Ok(())
            }
        },
        _ => Err(Error::NotApplicable),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
struct Args {
    info: LoadProjectInfo,
}

#[cfg(feature = "frontend")]
pub async fn query(info: LoadProjectInfo) -> Result<(), Error> {
    use crate::invoke::*;
    invoke_query::<(), Error, Args>("load_project", &Args { info }).await
}
