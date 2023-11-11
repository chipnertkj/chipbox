#[cfg(feature = "backend")]
use chipbox_backend_lib as backend_lib;
use chipbox_common as common;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum Error {
    NotApplicable,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`set_settings` is N/A in this context")
    }
}

#[cfg(feature = "backend")]
#[tauri::command]
pub(crate) async fn set_settings(
    state: tauri::State<'_, backend_lib::ManagedApp>,
    settings: common::Settings,
) -> Result<(), Error> {
    let mut backend_app = state.inner().lock().await;
    let configured_state_opt = backend_app.as_configured_state_mut();
    match configured_state_opt {
        Some(configured_state) => *configured_state.settings_mut() = settings,
        None => return Err(Error::NotApplicable),
    }
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
struct Args {
    settings: common::Settings,
}

#[cfg(feature = "frontend")]
pub async fn query(settings: common::Settings) -> Result<(), Error> {
    use crate::invoke::*;
    invoke_query::<(), Error, Args>("set_settings", &Args { settings }).await
}
