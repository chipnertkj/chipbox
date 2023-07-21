use std::time::{Duration, SystemTime};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

fn system_time() -> SystemTime {
    fn perf_to_system(amt: f64) -> SystemTime {
        let secs = (amt as u64) / 1_000;
        let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000;
        std::time::UNIX_EPOCH + Duration::new(secs, nanos)
    }
    let performance_now = web_sys::window()
        .expect("window should exist")
        .performance()
        .expect("performance is not available")
        .now();
    perf_to_system(performance_now)
}

pub(crate) async fn invoke_query<
    T: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    E: for<'de> serde::Deserialize<'de> + std::fmt::Debug,
    Args: serde::Serialize + std::fmt::Debug,
>(
    cmd: &str,
    args: &Args,
) -> Result<T, E> {
    tracing::info!("{cmd}({args:?})");
    let time_begin = system_time();
    let result = invoke(cmd, serde_wasm_bindgen::to_value(args).unwrap()).await;
    if let Ok(elapsed) = system_time().duration_since(time_begin) {
        tracing::info!("{cmd}({args:?}): {elapsed:?} elapsed");
    }
    match result {
        Ok(js_value) => {
            let value = serde_wasm_bindgen::from_value(js_value)
                .expect("invalid response: unable to deserialize Ok(js_value)");
            tracing::info!("{cmd}({args:?}) -> Ok({value:?})");
            Ok(value)
        }
        Err(js_value) => {
            let value = serde_wasm_bindgen::from_value(js_value).expect(
                "invalid response: unable to deserialize Err(js_value)",
            );
            tracing::error!("{cmd}({args:?}) -> Err({value:?})");
            Err(value)
        }
    }
}
