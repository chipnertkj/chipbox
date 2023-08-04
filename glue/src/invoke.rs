//! Defines a frontend interface for calling backend commands.

use wasm_bindgen::prelude::*;
use wasm_timer::Instant;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    /// Run a `tauri` commmand on the backend.
    /// The command has to be registered as a `tauri` command.
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

/// Call a backend command on `chipbox-backend` and retrieve the result.
/// The command has to be registered as a `tauri` command.
///
/// # Panics
/// If the function signature (note the generic parameters) does not match the specified backend command, this function
/// will panic.
pub(crate) async fn invoke_query<T, E, Args>(
    cmd: &str,
    args: &Args,
) -> Result<T, E>
where
    T: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    E: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    Args: serde::Serialize + std::fmt::Debug,
{
    // Start perf timer.
    tracing::info!("{cmd}({args:?})");
    let instant_begin = Instant::now();

    // Call backend.
    let result = invoke(cmd, serde_wasm_bindgen::to_value(args).unwrap()).await;

    // End perf timer.
    let elapsed = Instant::now().duration_since(instant_begin);
    tracing::info!("{cmd}({args:?}): {elapsed:?} elapsed");

    // validate and return
    match result {
        Ok(js_value) => {
            let value = serde_wasm_bindgen::from_value(js_value)
                .expect(&format!("invalid response: unable to deserialize Ok(js_value). does the function signature match `{cmd}`?"));
            tracing::info!("{cmd}({args:?}) -> Ok({value:?})");
            Ok(value)
        }
        Err(js_value) => {
            let value = serde_wasm_bindgen::from_value(js_value)
                .expect(&format!("invalid response: unable to deserialize Err(js_value). does the function signature match `{cmd}`?"));
            tracing::error!("{cmd}({args:?}) -> Err({value:?})");
            Err(value)
        }
    }
}
