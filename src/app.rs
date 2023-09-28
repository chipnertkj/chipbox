mod querying_backend;
mod setup;

use chipbox_glue as glue;
use querying_backend::QueryingBackend;
use setup::Setup;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(PartialEq, Default)]
enum RenderState {
    #[default]
    Requested,
    Idle,
}

#[derive(Clone, PartialEq)]
pub struct RerenderCallback {
    pub inner: Callback<()>,
}

#[function_component]
pub(super) fn App() -> yew::Html {
    let app_state = use_state(glue::App::default);
    let render_state = use_state(RenderState::default);

    // Query app state if RenderState::Requested.
    use_memo(
        |render_state| {
            if **render_state == RenderState::Requested {
                query_app_state(app_state.clone());
                render_state.set(RenderState::Idle);
            }
        },
        render_state.clone(),
    );

    let rerender_cb = RerenderCallback {
        inner: Callback::from(move |_: ()| {
            render_state.set(RenderState::Requested);
        }),
    };

    let inner_html = match &*app_state {
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
    };

    html! {
        <ContextProvider<RerenderCallback> context={rerender_cb}>
            {inner_html}
        </ContextProvider<RerenderCallback>>
    }
}

fn query_app_state(state_handle: yew::UseStateHandle<glue::App>) {
    spawn_local(async move {
        let app = glue::app::query().await;
        state_handle.set(app);
    })
}
