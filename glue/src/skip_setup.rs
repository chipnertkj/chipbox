#[cfg(feature = "backend")]
use chipbox_backend_lib as backend_lib;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum Error {
    NotApplicable,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`skip_setup` is N/A in this context")
    }
}

#[cfg(feature = "backend")]
#[tauri::command]
pub(crate) async fn skip_setup(
    state: tauri::State<'_, backend_lib::ManagedApp>,
) -> Result<(), Error> {
    let mut backend_app = state.arc.lock().await;
    match &mut *backend_app {
        backend_lib::App::Setup(setup) => {
            if let backend_lib::Setup::First = setup {
                *backend_app = backend_lib::App::ProjectSelection(
                    backend_lib::ProjectSelection {
                        settings: Default::default(),
                    },
                );
                Ok(())
            } else {
                Err(Error::NotApplicable)
            }
        }
        _ => Err(Error::NotApplicable),
    }
}

#[cfg(feature = "frontend")]
pub async fn query() -> Result<(), Error> {
    use crate::invoke::*;
    invoke_query::<(), Error, ()>("skip_setup", &()).await
}
