mod editor;
mod home;
mod title_bar;
mod wizard;

use chipbox_glue::cmd;
use editor::Editor;
use home::Home;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use title_bar::TitleBar;
use wizard::Wizard;

fn fetch() -> impl IntoView {
    // TODO: add information on reporting issues.
    view! {
        <div class="fetch-container">
            <h1>"Fetching current state from the backend..."</h1>
        </div>
    }
}

fn error_retry(title: &'static str, msg: String) -> impl IntoView {
    // TODO: add information on reporting issues.
    view! {
        <div class="error-container">
            <h1>{title}</h1>
            <code class="error-message">{msg}</code>
            <p>"Press the button to retry"</p>
            <button on:click=move |_| {
                tracing::info!("reloading the page");
                window().location().reload().unwrap();
            }>"⟳"</button>
        </div>
    }
}

fn content() -> impl IntoView {}

#[component]
pub(crate) fn App() -> impl IntoView {
    let project = signal(None);

    view! {
        <TitleBar />
        <main>{move || content()}</main>
    }
}
