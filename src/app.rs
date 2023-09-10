mod querying_backend;
mod setup;

use chipbox_glue as glue;
use querying_backend::QueryingBackend;
use setup::Setup;
use yew::platform::spawn_local;
use yew::prelude::*;

fn query_state(app_state: yew::UseStateHandle<glue::App>) {
    spawn_local(async move {
        let app = glue::app::query().await;
        app_state.set(app);
    })
}

#[function_component]
pub(super) fn App() -> yew::Html {
    let app_state = use_state(glue::App::default);
    use_memo(|_| query_state(app_state.clone()), ());

    match &*app_state {
        glue::App::QueryingBackend(state) => html! {
            <QueryingBackend state={*state} />
        },
        glue::App::Setup(state) => html! {
            <Setup state={state.clone()} />
        },
        glue::App::Home => html! {
            <h1>{"Home"}</h1>
        },
        glue::App::Editor => html! {
            <h1>{"Editor"}</h1>
        },
    }
}
