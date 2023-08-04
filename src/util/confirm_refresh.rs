//! Utility functions for managing the confirmation dialog
//! for when the user tries to refresh the page.

use js_sys::Function;
use wasm_bindgen::prelude::*;

/// Enable a confirmation dialog for when the user tries to refresh the page.
///
/// Adds an event listener for the `beforeunload` event and sets a return value
/// for the event to display a confirmation message to the user.
pub(crate) fn enable_confirm_refresh() {
    // Define `beforeunload` callback.
    let cb: Function = Closure::once_into_js(
        move |ev: web_sys::BeforeUnloadEvent| -> String {
            ev.prevent_default();
            ev.set_return_value("Are you sure you want to exit? Refreshing will discard your changes.");
            ev.return_value()
        },
    )
    .into();
    // Add event listener.
    gloo::utils::window()
        .add_event_listener_with_callback("beforeunload", &cb)
        .expect("failed to add event listener");
}
