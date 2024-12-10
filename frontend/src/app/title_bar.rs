use crate::tauri_api;
use button::{Button, State};
use leptos::{prelude::*, task};
use leptos_use::{use_event_listener, use_window};

mod button;

/// Custom window title bar.
#[component]
pub(crate) fn TitleBar() -> impl IntoView {
    let maximized = Maximized::register();

    view! {
        <div id="title-bar" on:contextmenu=move |e| e.prevent_default()>
            // TODO: title bar context-dependent content
            <div id="title-bar-content"></div>
            <div id="title-bar-buttons">
                <div class="title-bar-separator" />
                <Button state=State::More />
                <Button state=State::Minimize />
                // Resize button changes behavior depending on maximized state.
                // It may also update the maximized signal when pressed.
                // NOTE: There are cases where a window may enter maximized state
                // without it being resized.
                // This is why we check for both resizing and pressing the button.
                {move || {
                    tracing::debug!("changing resize button state / rendering");
                    let state = if (maximized.memo)() {
                        State::Restore(maximized.set)
                    } else {
                        State::Maximize(maximized.set)
                    };
                    view! { <Button state=state /> }
                }}
                <Button state=State::Close />
            </div>
        </div>
    }
}

/// Memoized signal for updating the resize button based on window state.
/// Automatically registers a callback on window resize,
/// updating the signal value.
struct Maximized {
    memo: Memo<bool>,
    set: WriteSignal<bool>,
}

impl Maximized {
    /// Register an event listener that updates a signal based on window state.
    ///
    /// Returns the signal and listener handle.
    fn register() -> Self {
        // Signals for updating the resize button.
        let (value, set) = signal(false);
        // Update `maximized` signal on window resize.
        let _stop = use_event_listener(use_window(), leptos::ev::resize, move |_| {
            // Safety: safe as long as the API remains stable.
            let window = unsafe { tauri_api::window::js_current_window() };
            task::spawn_local(async move { set(window.is_maximized().await) })
        });
        tracing::debug!("registered window resize listener");
        // Construct memo, return handle ownership.
        let memo = Memo::new(move |_| value());
        Self { memo, set }
    }
}
