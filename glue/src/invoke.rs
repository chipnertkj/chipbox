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
    /// The command has to be registered as a `tauri` command.
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

/// Call a backend command on `chipbox-backend` and retrieve the result.
/// The command has to be registered as a `tauri` command.
///
/// Returns the exact value returned by the backend command.
/// If the command returns a `Result`, use `invoke_query` instead.
///
/// # Panics
/// - Panics if the function signature (note the generic parameters) does not
/// match that of the specified backend command.
/// - Panics if the command name `cmd` does not match any command on the backend.
pub(crate) async fn invoke_query_infallible<T, Args>(
    cmd: &str,
    args: &Args,
) -> T
where
    T: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    Args: serde::Serialize + std::fmt::Debug,
{
    invoke_query::<T, Infallible, Args>(cmd, args)
        .await
        .expect("infallible")
}

/// Call a backend command on `chipbox-backend` and retrieve the result.
/// The command has to be registered as a `tauri` command.
///
/// Returns a `Result`, as defined in the backend command`s function signature.
/// If the command does not return a `Result`, use `invoke_query_infallible` instead.
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
    let cmd_pretty = format!("`{cmd}`({args:?})");

    // Start perf timer.
    tracing::trace!("{cmd_pretty}: begin query");
    let instant_begin = Instant::now();

    let backend_result = invoke(
        cmd,
        serde_wasm_bindgen::to_value(args)
            .expect("{cmd_pretty}: Unable to serialize args"),
    )
    .await;

    // End perf timer.
    let elapsed = Instant::now().duration_since(instant_begin);
    tracing::trace!("{cmd_pretty}: {elapsed:?} elapsed");

    match backend_result {
        Ok(js_value) => {
            let value = serde_wasm_bindgen::from_value(js_value)
                .unwrap_or_else(|_| {
                    panic!(
                        "{cmd_pretty}: Invalid response - type mismatch. \
                        Unable to deserialize `Ok(T)`. \
                        Does the function signature match `{cmd}`?"
                    );
                });
            tracing::info!("{cmd_pretty} -> Ok({value:?})");
            Ok(value)
        }
        Err(js_value) => {
            let value = serde_wasm_bindgen::from_value(js_value)
                .unwrap_or_else(|e| {
                    tracing::error!(
                        "{cmd_pretty}: unable to deserialize `Err(E)`: {e}"
                    );
                    // The fallback is needed due to `tauri` command jank.
                    tracing::warn!(
                        "{cmd_pretty}: falling back to `Err(String)`"
                    );
                    let err: String = serde_wasm_bindgen::from_value(e.into())
                        .unwrap_or_else(|e| {
                            tracing::error!(
                                "{cmd_pretty}: unable to deserialize `Err(String)`: {e}"
                            );
                            panic!(
                                "{cmd_pretty}: Invalid response - type mismatch. \
                                Unable to deserialize `Err(String)`. \
                                Does the function signature match `{cmd}`?"
                            );
                        });
                    panic!("{cmd_pretty}: {err:?}");
                });
            tracing::error!("{cmd_pretty} -> Err({value:?})");
            Err(value)
        }
    }
}
