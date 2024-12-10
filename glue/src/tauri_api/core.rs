//! Relevant definitions from the [`core`](https://v2.tauri.app/reference/javascript/api/namespacecore/)
//! namespace of the [Tauri JavaScript API](https://v2.tauri.app/reference/javascript/api/).

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;
use web_time::Instant;

#[wasm_bindgen]
unsafe extern "C" {
    /// https://v2.tauri.app/reference/javascript/api/namespacecore/#invoke
    ///
    /// Run a `tauri` commmand on the backend.
    /// The command has to be registered as a `tauri` command in the command handler.
    ///
    /// # Safety
    /// This function is safe to execute as long as the function signature matches
    /// the [Tauri JavaScript API](https://v2.tauri.app/reference/javascript/api/).
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = "invoke")]
    async unsafe fn js_invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

pub(crate) type InvokeResult<'a, T, Args> = std::result::Result<T, InvokeError<'a, Args>>;

/// An error encountered while invoking a command on the backend.
#[derive(thiserror::Error, Debug)]
pub enum InvokeError<'a, Args: Debug> {
    /// Encountered a JS exception while executing the command.
    #[error("backend returned a js exception: {0:?}")]
    Exception(JsValue),
    /// Unable to serialize arguments provided to the command.
    #[error("unable to serialize arguments `{args:?}`: {e}")]
    Serialize {
        e: serde_wasm_bindgen::Error,
        args: &'a Args,
    },
    /// Unable to deserialize the response returned by the backend.
    #[error("unable to deserialize response: {0}")]
    Deserialize(serde_wasm_bindgen::Error),
}

/// Invoke a command on the backend.
///
/// The command is assumed to be registered as a [`tauri`] command in the command handler.
/// See the [`handler`](crate::handler) module for more information.
///
/// May return an error if serialization or deserialization fails, or if a JavaScript exception occurs.
/// This can happen if, for example, the function attempts to call a command that is not registered as a `tauri` command.
pub(crate) async fn invoke<'a, Args, T>(
    cmd: &'static str,
    args: &'a Args,
) -> Result<T, InvokeError<'a, Args>>
where
    Args: Serialize + Debug,
    T: for<'de> Deserialize<'de> + Debug,
{
    tracing::debug!("{cmd}({args:?})");
    // Serialize query arguments.
    let js_args =
        serde_wasm_bindgen::to_value(args).map_err(|e| InvokeError::Serialize { args, e })?;

    // Start perf timer.
    let instant_begin = Instant::now();
    // Invoke backend command.
    // Safety: safe as long as the API remains stable.
    let js_response = unsafe { js_invoke(cmd, js_args) }.await;
    // End perf timer.
    let elapsed = Instant::now().duration_since(instant_begin);
    tracing::debug!("{cmd}({args:?}): done in {elapsed:?}");

    // Deserialize response.
    tracing::trace!("{cmd}({args:?}): js returned {js_response:?}");
    let js_value = js_response.map_err(InvokeError::Exception)?;
    let value = serde_wasm_bindgen::from_value(js_value).map_err(InvokeError::Deserialize)?;
    tracing::trace!("{cmd}({args:?}): deserializes into {value:?}");
    // Return result.
    tracing::info!("{cmd}({args:?}) -> {value:?}");
    Ok(value)
}
