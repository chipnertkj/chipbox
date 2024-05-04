use self::backend_query::BackendQuery;
use self::home::Home;
use self::setup::Setup;
use self::state::AppState;
use yew::prelude::*;

mod backend_query;
mod editor;
mod home;
mod setup;
mod state;

#[function_component]
pub fn App() -> yew::Html {
    // App state.
    let app_state = use_state_eq(AppState::default);
    // After rendering, query the backend.
    use_effect({
        let app_state = app_state.clone();
        move || query_backend_app_state(app_state)
    });

    match &*app_state {
        AppState::BackendQuery(ref backend_query_state) => html! {
            <BackendQuery state={*backend_query_state} />
        },
        AppState::Setup(ref setup_state) => html! {
            <Setup state={*setup_state} />
        },
        AppState::Home(ref home_state) => html! {
            <Home state={home_state.clone()} />
        },
        AppState::BackendClosed => html_backend_closed(),
    }
}

fn html_backend_closed() -> Html {
    html! {
        <div>
            <h1 class="primary">{"Backend app thread channel closed"}</h1>
            <p class="primary">{"Unable to deliver query message to backend app thread - thread channel was closed."}</p>
            <p class="secondary">{"See backend logs for details."}</p>
        </div>
    }
}

fn query_backend_app_state(app_state: UseStateHandle<AppState>) {
    unreachable!()
}
