use chipbox_glue as glue;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use yew::platform::spawn_local;
use yew::prelude::*;
mod setup;
use setup::Setup;

/// Initialize `console_error_panic_hook` on debug.
/// This forwards Rust panic traces to the console.
fn setup_panic_console_hook() {
    if cfg!(debug_assertions) {
        console_error_panic_hook::set_once();
    }
}

/// Initialize `tracing`, `tracing_subscriber` and `tracing_web`.
fn setup_tracing() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        // WebViews do not support time.
        .without_time()
        .with_writer(tracing_web::MakeConsoleWriter);
    let performance_layer = tracing_web::performance_layer()
        .with_details_from_fields(
            tracing_subscriber::fmt::format::Pretty::default(),
        );
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(performance_layer)
        .init();
}

/// Application entry point.
fn main() {
    setup_panic_console_hook();
    setup_tracing();
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> yew::Html {
    use glue::app::{App, QueryingBackend};

    let app_state = use_state(App::default);
    use_memo(
        |_| {
            let app_state = app_state.clone();
            spawn_local(async move {
                let app = glue::app::query().await;
                app_state.set(app);
            })
        },
        (),
    );

    match &*app_state {
        App::QueryingBackend(state) => match state {
            QueryingBackend::Requesting => html! {
                <h1>{"Requesting state"}</h1>
            },
            QueryingBackend::ReadingSettings => html! {
                <h1>{"Reading Settings"}</h1>
            },
        },
        App::Setup(state) => html! { <Setup state={state.clone()} /> },
        App::Home => html! {
            <h1>{"Home"}</h1>
        },
        App::Editor => html! {
            <h1>{"Editor"}</h1>
        },
    }
}
