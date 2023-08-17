use chipbox_glue as glue;
use setup::Setup;
use yew::platform::spawn_local;
use yew::prelude::*;
mod setup;

#[function_component]
pub(super) fn App() -> yew::Html {
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
