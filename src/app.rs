use chipbox_glue as glue;
use setup::Setup;
use yew::platform::spawn_local;
use yew::prelude::*;
mod querying_backend;
mod setup;
use querying_backend::QueryingBackend;

#[function_component]
pub(super) fn App() -> yew::Html {
    let app_state = use_state(glue::App::default);
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
        glue::App::QueryingBackend(state) => html! {
            <QueryingBackend state={state.clone()} />
        },
        glue::App::Setup(state) => html! { <Setup state={state.clone()} /> },
        glue::App::Home => html! {
            <h1>{"Home"}</h1>
        },
        glue::App::Editor => html! {
            <h1>{"Editor"}</h1>
        },
    }
}
