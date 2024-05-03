use crate::{common, glue};
use common::app::{BackendMsg, FrontendMsg};
use yew::platform::spawn_local;
use yew::prelude::*;

mod editor;
mod home;
mod querying_backend;
mod setup;

use querying_backend::QueryingBackend;

impl From<common::app::State> for State {
    fn from(value: common::app::State) -> Self {
        use common::app::{AwaitConfig, State};

        match value {
            State::ReadingSettings => {
                Self::QueryingBackend(querying_backend::State::ReadingSettings)
            }
            State::AwaitConfig(awaiting_config) => match awaiting_config {
                AwaitConfig::NoConfig => Self::Setup(setup::State::First),
            },
            State::Idle => Self::Home(home::State::QueryingSettings),
            State::Editor => todo!(),
        }
    }
}

#[derive(PartialEq)]
enum State {
    QueryingBackend(querying_backend::State),
    Setup(setup::State),
    Home(home::State),
    BackendClosed,
}

impl Default for State {
    fn default() -> Self {
        Self::QueryingBackend(Default::default())
    }
}

fn on_reading_settings(state: UseStateHandle<State>) {
    state.set(State::QueryingBackend(
        querying_backend::State::ReadingSettings,
    ));
}

fn on_query_app_response(
    state: UseStateHandle<State>,
    app_state: common::app::State,
) {
    state.set(app_state.into());
}

fn handle_backend_message(
    state: UseStateHandle<State>,
    event: tauri_sys::event::Event<BackendMsg>,
) {
    let msg = event.payload;
    tracing::trace!("Received backend message: {:?}", msg);
    match msg {
        common::app::BackendMsg::ReadingSettings => on_reading_settings(state),
        common::app::BackendMsg::QueryAppResponse(app_state) => {
            on_query_app_response(state, app_state)
        }
    }
}

fn effect(state: UseStateHandle<State>) {
    // Add event listener for backend messages.
    spawn_local({
        let state = state.clone();
        async move {
            // We listen to only one event per render.
            let res = tauri_sys::event::once(BackendMsg::event_name()).await;
            match res {
                Ok(msg) => handle_backend_message(state, msg),
                Err(err) => tracing::error!(
                    "Failed to listen for backend messages: {:?}",
                    err
                ),
            }
        }
    });
    // Send `QueryApp` message.
    spawn_local(async move {
        if !glue::msg::send(FrontendMsg::QueryApp).await {
            state.set(State::BackendClosed);
        }
    });
}

#[function_component]
pub fn App() -> yew::Html {
    // App state.
    let state = use_state_eq(State::default);
    // After rendering, query the backend.
    use_effect({
        let state = state.clone();
        move || effect(state)
    });

    match &*state {
        State::QueryingBackend(state) => html! {
            <QueryingBackend state={*state} />
        },
        State::Setup(state) => html! {
            <setup::Setup state={*state} />
        },
        State::Home(state) => html! {
            <home::Home state={state.clone()} />
        },
        State::BackendClosed => html! {
            <p class="text primary">{"Backend thread closed before reply. See backend logs for details."}</p>
        },
    }
}
