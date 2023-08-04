use chipbox_backend_lib::state::State;

#[cfg(feature = "backend")]
use chipbox_backend_lib::state::ManagedState;
#[cfg(feature = "frontend")]
use yew::AttrValue;

#[cfg(feature = "backend")]
#[tauri::command]
pub(crate) async fn state(
    state: tauri::State<'_, ManagedState>,
) -> Result<State, String> {
    let result = state.mx.lock().await;
    match result.as_ref() {
        Ok(app_state) => Ok(app_state.clone()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(feature = "frontend")]
pub async fn query() -> Query {
    use crate::invoke::invoke_query;

    let result = invoke_query::<State, String, ()>("state", &())
        .await
        .map_err(Into::into);
    Query { result }
}

#[cfg(feature = "frontend")]
pub struct Query {
    pub result: Result<State, AttrValue>,
}
