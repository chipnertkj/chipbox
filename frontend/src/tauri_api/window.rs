use wasm_bindgen::prelude::*;

#[allow(non_snake_case)]
#[wasm_bindgen]
unsafe extern "C" {
    /// <https://v2.tauri.app/reference/javascript/api/namespacewindow/#window>
    /// # Safety
    /// This function is safe to execute as long as the function signature matches
    /// the [Tauri JavaScript API](https://v2.tauri.app/reference/javascript/api/).
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"], js_name = "Window")]
    pub(crate) type JsWindow;

    /// <https://v2.tauri.app/reference/javascript/api/namespacewindow/#getcurrentwindow>
    /// # Safety
    /// This function is safe to execute as long as the function signature matches
    /// the [Tauri JavaScript API](https://v2.tauri.app/reference/javascript/api/).
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"], js_name = "getCurrentWindow")]
    pub(crate) unsafe fn js_current_window() -> JsWindow;

    /// <https://v2.tauri.app/reference/javascript/api/namespacewindow/#close>
    /// # Safety
    /// This function is safe to execute as long as the function signature matches
    /// the [Tauri JavaScript API](https://v2.tauri.app/reference/javascript/api/).
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"], method, js_name = "close")]
    pub(crate) async unsafe fn js_close(this: &JsWindow);

    /// <https://v2.tauri.app/reference/javascript/api/namespacewindow/#ismaximized>
    /// # Note
    /// This function returns a [`JsValue`].
    /// Use the more rusty [`is_maximized`][Window::is_maximized].
    /// # Safety
    /// This function is safe to execute as long as the function signature matches
    /// the [Tauri JavaScript API](https://v2.tauri.app/reference/javascript/api/).
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"], method, js_name = "isMaximized")]
    async unsafe fn js_is_maximized(this: &JsWindow) -> JsValue;

    /// <https://v2.tauri.app/reference/javascript/api/namespacewindow/#maximize>
    /// # Safety
    /// This function is safe to execute as long as the function signature matches
    /// the [Tauri JavaScript API](https://v2.tauri.app/reference/javascript/api/).
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"], method, js_name = "toggleMaximize")]
    pub(crate) async unsafe fn js_toggle_maximize(this: &JsWindow);

    /// <https://v2.tauri.app/reference/javascript/api/namespacewindow/#minimize>
    /// # Safety
    /// This function is safe to execute as long as the function signature matches
    /// the [Tauri JavaScript API](https://v2.tauri.app/reference/javascript/api/).
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"], method, js_name = "minimize")]
    pub(crate) async unsafe fn js_minimize(this: &JsWindow);
}

impl JsWindow {
    /// Check if the window is currently maximized.
    ///
    /// The resulting value is deserialized from a [`JsValue`].
    /// This function will panic if the underlying API does not return a boolean.
    /// See [`JsWindow::js_is_maximized`].
    pub(crate) async fn is_maximized(&self) -> bool {
        // Safety: safe as long as the API remains stable.
        let is_maximized = unsafe { self.js_is_maximized() }.await;
        serde_wasm_bindgen::from_value(is_maximized).expect("is_maximized from js should give bool")
    }
}
