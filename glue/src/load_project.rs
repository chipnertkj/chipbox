#[cfg(feature = "backend")]
use chipbox_backend_lib as backend_lib;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum Error {
    NotApplicable,
    AudioEngine(String),
    PlayStream(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotApplicable => {
                write!(f, "`load_project` is N/A in this context")
            }
            Self::AudioEngine(e) => write!(f, "editor error: {e}"),
            Self::PlayStream(e) => write!(f, "playback error: {e}"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum LoadProjectInfo {
    New { name: String },
    Load(std::path::PathBuf),
}

#[cfg(feature = "backend")]
#[tauri::command]
pub(crate) fn load_project(
    state: tauri::State<'_, backend_lib::ManagedApp>,
    info: LoadProjectInfo,
) -> Result<(), Error> {
    use tauri::async_runtime;

    let mut backend_app = async_runtime::block_on(state.arc.lock());
    match &mut *backend_app {
        backend_lib::App::ProjectSelection(project_selection) => match info {
            LoadProjectInfo::New { name } => {
                let settings = project_selection
                    .settings
                    .clone();
                let editor_result =
                    backend_lib::Editor::create_project(settings, name);
                match editor_result {
                    Ok(editor) => {
                        *backend_app =
                            backend_lib::App::Editor(Box::new(editor));
                        Ok(())
                    }
                    Err(backend_lib::editor::Error::PlayStream {
                        editor,
                        e,
                    }) => {
                        *backend_app = backend_lib::App::Editor(editor);
                        Err(Error::PlayStream(e.to_string()))
                    }
                    Err(backend_lib::editor::Error::AudioEngine(e)) => {
                        Err(Error::AudioEngine(e.to_string()))
                    }
                }
            }
            LoadProjectInfo::Load(_project_path) => {
                todo!();
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
