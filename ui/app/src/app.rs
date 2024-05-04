use crate::{common, glue};
use common::app::{BackendMsg, FrontendMsg, FrontendQuery};
use yew::platform::spawn_local;
use yew::prelude::*;

mod editor;
mod home;
mod querying_backend;
mod setup;

use querying_backend::QueryingBackend;

impl From<common::app::BackendAppState> for AppState {
    fn from(value: common::app::BackendAppState) -> Self {
        use common::app::{AwaitConfigReason, BackendAppState};

        match value {
            BackendAppState::ReadingSettings => {
                Self::QueryingBackend(querying_backend::State::ReadingSettings)
            }
            BackendAppState::AwaitConfig { reason } => match reason {
                AwaitConfigReason::NoConfig => Self::Setup(setup::State::First),
            },
            BackendAppState::Idle => Self::Home(home::State::QueryingSettings),
            BackendAppState::Editor => todo!(),
        }
    }
}

#[derive(PartialEq)]
enum AppState {
    QueryingBackend(querying_backend::State),
    Setup(setup::State),
    Home(home::State),
    BackendClosed,
}

impl Default for AppState {
    fn default() -> Self {
        Self::QueryingBackend(Default::default())
    }
}

fn on_reading_settings(state: UseStateHandle<AppState>) {
    state.set(AppState::QueryingBackend(
        querying_backend::State::ReadingSettings,
    ));
}

fn on_query_app_response(
    state_handle: UseStateHandle<AppState>,
    app_state: common::app::BackendAppState,
) {
    state_handle.set(app_state.into());
}

fn handle_backend_message(
    state_handle: UseStateHandle<AppState>,
    event: tauri_sys::event::Event<BackendMsg>,
) {
    let msg = event.payload;
    tracing::trace!("Received backend message: {:?}", msg);
    match msg {
        common::app::BackendMsg::ReadingSettings => {
            on_reading_settings(state_handle)
        }
        // There will be more responses in the future.
        // Warning is irrelevant.
        #[allow(irrefutable_let_patterns)]
        common::app::BackendMsg::Response(response) => match response {
            common::app::BackendResponse::BackendAppState(app_state) => {
                on_query_app_response(state_handle, app_state)
            }
            common::app::BackendResponse::Settings(_settings) => {
                unimplemented!()
            }
        },
    }
}

fn effect(state: UseStateHandle<AppState>) {
    // Add event listener for backend messages.
    spawn_local({
        let state = state.clone();
        async move {
            // We listen to only one event per render.
            let result = tauri_sys::event::once(BackendMsg::event_name()).await;
            match result {
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
        if !glue::msg::send(FrontendMsg::Query(FrontendQuery::BackendAppState))
            .await
        {
            state.set(AppState::BackendClosed);
        }
    });
}

#[function_component]
pub fn App() -> yew::Html {
    // App state.
    let state = use_state_eq(AppState::default);
    // After rendering, query the backend.
    use_effect({
        let state = state.clone();
        move || effect(state)
    });

    match &*state {
        AppState::QueryingBackend(state) => html! {
            <QueryingBackend state={*state} />
        },
        AppState::Setup(state) => html! {
            <setup::Setup state={*state} />
        },
        AppState::Home(state) => html! {
            <home::Home state={state.clone()} />
        },
        AppState::BackendClosed => html! {
            <div>
                <h1 class="primary">{"Backend app thread channel closed"}</h1>
                <p class="primary">{"Unable to deliver query message to backend app thread - thread channel was closed."}</p>
                <p class="secondary">{"See backend logs for details."}</p>
            </div>
        },
    }
}
