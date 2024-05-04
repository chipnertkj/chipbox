//! Defines a frontend interface for calling backend commands.

mod infallible;

use infallible::Infallible;
use wasm_bindgen::prelude::*;
use wasm_timer::Instant;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    /// Run a `tauri` commmand on the backend.
    /// The command has to be registered as a `tauri` command in the command handler.
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

/// Call a backend command on `chipbox-backend` and retrieve the returned value.
/// The command has to be registered as a `tauri` command.
///
/// Returns the exact value retrieved from the backend command.
/// If the command returns a `Result`, use `invoke_query` instead.
/// Failure to do so may cause a type-mismatch, which will result in a panic.
///
/// # Panics
/// - Panics if the function signature (note the generic parameters) does not
/// match that of the specified backend command. This also applies if the
/// command natively returns a `Result`.
/// `invoke` returns a JS Promise error if the command does not exist.
/// - Panics if the command name `cmd` does not match any command on the backend.
pub(crate) async fn invoke_query_infallible<T, Args>(
    cmd: &str,
    args: &Args,
) -> T
where
    T: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    Args: serde::Serialize + std::fmt::Debug,
{
    // Invoke query with private `Infallible` type.
    invoke_query::<T, Infallible, Args>(cmd, args)
        .await
        // If this happens, you're misusing the function.
        .expect("infallible")
}

/// Call a backend command on `chipbox-backend` and retrieve the result.
/// The command has to be registered as a `tauri` command.
///
/// Returns a `Result`, as defined in the backend command's function signature.
/// If the command does not return a `Result`, use `invoke_query_infallible` instead.
///
/// # TODO
/// - Explain why we can't use `invoke_query` for infallible queries.
///
/// # Panics
/// - Panics if the function signature (note the generic parameters) does not
/// match that of the specified backend command.
/// - Panics if the command name `cmd` does not match any command on the backend.
pub(crate) async fn invoke_query<T, E, Args>(
    cmd: &str,
    args: &Args,
) -> Result<T, E>
where
    T: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    E: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    Args: serde::Serialize + std::fmt::Debug,
{
    // Format debug command name.
    let cmd_pretty = format!("{cmd}({args:?})");

    // Start perf timer.
    tracing::trace!("{cmd_pretty}: Begin query.");
    let instant_begin = Instant::now();

    // Serialize query arguments.
    let args = serde_wasm_bindgen::to_value(args)
        // Invalid arguments struct.
        // Verify that serialization is set up correctly.
        .expect("{cmd_pretty}: Unable to serialize query arguments.");
    // Invoke backend command.
    let backend_result = invoke(cmd, args).await;

    // End perf timer.
    let elapsed = Instant::now().duration_since(instant_begin);
    tracing::trace!("{cmd_pretty}: elapsed: {elapsed:?}");
    tracing::trace!("{cmd_pretty}: Response: `{backend_result:?}`");

    // Handle response.
    match backend_result {
        Ok(js_value) => {
            // Deserialize `Ok(T)` response.
            let value = serde_wasm_bindgen::from_value(js_value)
                .unwrap_or_else(|err| {
                    // Type mismatch.
                    tracing::error!(
                        "{cmd_pretty}: Unable to deserialize `Ok(T)`: {err}"
                    );
                    panic!(
                        "{cmd_pretty}: Invalid response - type mismatch. \
                        Unable to deserialize `Ok(T)`. \
                        Does the function signature match `{cmd}`?",
                    );
                });
            // Deserialization ok.
            tracing::info!("{cmd_pretty} -> Ok({value:?})");
            Ok(value)
        }
        Err(js_value) => {
            // Deserialize error response.
            // `js_value` is cloned so that we can later print the error trace
            // in case of a missing command handler.
            let value = serde_wasm_bindgen::from_value(js_value.clone())
                .unwrap_or_else(|err| {
                    // Error cause may be either type mismatch or missing
                    // command handler. A missing command handler will
                    // cause `invoke` to return a JS Promise error.
                    tracing::error!(
                        "{cmd_pretty}: Unable to deserialize error response: {err}"
                    );
                    // The fallback is needed due to command jank.
                    // See comment at scope root.
                    tracing::warn!(
                        "{cmd_pretty}: Unable to deserialize error value. Falling back to `Err(JsValue)`"
                    );
                    tracing::warn!(
                        "{cmd_pretty}: Printing error trace from the captured `JsValue`..."
                    );
                    tracing::error!("{cmd_pretty}: Root cause was `{js_value:?}`");
                    panic!(
                        "{cmd_pretty}: Invalid response - type mismatch. \
                        Does the function signature match `{cmd}`? \
                        Is the command available from the command handler?"
                    );
                });
            // Deserialization ok.
            tracing::error!("{cmd_pretty} -> Err({value:?})");
            Err(value)
        }
    }
}
