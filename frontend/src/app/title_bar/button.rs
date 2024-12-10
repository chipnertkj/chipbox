use crate::tauri_api;
use leptos::{prelude::*, task};

#[component]
/// A button that implements title bar functionality.
pub(crate) fn Button(state: State) -> impl IntoView {
    let path_data = state.path_data();
    let stroke = state.stroke();
    let class = state.class();
    let on_click = move |_| state.on_click();

    view! {
        <button class=class on:click=on_click tabindex=-1>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <path d=path_data fill="currentColor" stroke=stroke />
            </svg>
        </button>
    }
}

#[derive(PartialEq, Clone, Copy)]
/// Holds current state of a [`Button`].
pub(crate) enum State {
    /// Show more options
    More,
    /// Minimize the window.
    Minimize,
    /// Maximize the window.
    ///
    /// Takes a reference to a signal representing the maximized state of the window.
    /// This signal will be updated whenever the button is pressed in this state.
    Maximize(WriteSignal<bool>),
    /// Restore the window.
    ///
    /// Takes a reference to a signal representing the maximized state of the window.
    /// This signal will be updated whenever the button is pressed in this state.
    Restore(WriteSignal<bool>),
    /// Request to close the window.
    Close,
}

impl State {
    /// SVG path data used for the buttons icon.
    fn path_data(&self) -> &'static str {
        match self {
            State::More => "M3 13v-2h2v2zm8 0v-2h2v2zm8 0v-2h2v2z",
            State::Minimize => "M4 11h16v2H4z",
            State::Maximize(_) => "M4 4h16v16H4zm2 4v10h12V8z",
            State::Restore(_) => "M4 8h4V4h12v12h-4v4H4zm12 0v6h2V6h-8v2zM6 12v6h8v-6z",
            State::Close => {
                "M13.46 12L19 17.54V19h-1.46L12 13.46L6.46
                19H5v-1.46L10.54 12L5 6.46V5h1.46L12
                10.54L17.54 5H19v1.46z"
            }
        }
    }

    /// SVG stroke used for the buttons icon.
    fn stroke(&self) -> Option<&'static str> {
        match self {
            State::More => Some("currentColor"),
            _ => None,
        }
    }

    /// CSS class used for the button.
    fn class(&self) -> &'static str {
        match self {
            State::Close => "title-bar-button-close",
            _ => "title-bar-button",
        }
    }

    /// Generates a click handler function for the button.
    /// The returned handler will be different depending on the state.
    fn on_click(self) {
        // Safety: safe as long as the API remains stable.
        let window = unsafe { tauri_api::window::js_current_window() };
        match self {
            State::More => todo!(),
            State::Minimize => Self::on_minimize(window),
            State::Maximize(set_maximized) | State::Restore(set_maximized) => {
                Self::on_resize(window, set_maximized)
            }
            State::Close => Self::on_close(window),
        }
    }

    /// Called when the minimize button is pressed.
    /// Minimizes the window.
    fn on_minimize(window: tauri_api::window::JsWindow) {
        task::spawn_local(async move {
            tracing::debug!("minimizing window");
            // Safety: safe as long as the API remains stable.
            unsafe { window.js_minimize() }.await;
        });
    }

    /// Called when the resize button is pressed.
    /// Toggles the maximized state of the window.
    /// Updates a signal with the new maximized state.
    fn on_resize(window: tauri_api::window::JsWindow, set_maximized: WriteSignal<bool>) {
        task::spawn_local(async move {
            tracing::debug!("toggling window maximized state");
            // Safety: safe as long as the API remains stable.
            unsafe { window.js_toggle_maximize() }.await;
            set_maximized(window.is_maximized().await);
        });
    }

    /// Called when the close button is pressed.
    /// Requests to close the window.
    fn on_close(window: tauri_api::window::JsWindow) {
        task::spawn_local(async move {
            tracing::debug!("requesting window to close");
            // Safety: safe as long as the API remains stable.
            unsafe { window.js_close() }.await;
        });
    }
}
