use super::querying_backend::{self, QueryingBackend};
use crate::common::Settings;
use const_format::formatc;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub(super) enum State {
    QueryingSettings,
    Welcome(Settings),
}

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: State,
}

#[function_component]
pub(super) fn Home(props: &Props) -> yew::Html {
    match &props.state {
        State::QueryingSettings => html_querying_settings(),
        State::Welcome(settings) => html_welcome(),
    }
}

fn html_querying_settings() -> yew::Html {
    html! {
        <QueryingBackend state={querying_backend::State::QueryingSettings} />
    }
}

fn html_welcome() -> Html {
    // On click new project.
    let on_click_new = move |_| {};

    html! {
        <main>
            <h1 class="title">
                {"chipbox"}
                <code class="header tertiary code">
                    {formatc!("v{}", env!("CARGO_PKG_VERSION"))}
                </code>
            </h1>
            <button onclick={on_click_new}>
                <h2 class="left">
                    {"Create a new project"}
                </h2>
                <p class="tertiary left">
                    {"Continue to the editor."}
                </p>
            </button>
            <button>
                <h2 class="left">{"User projects"}</h2>
                <p class="tertiary left">
                    {"Browse projects in the user directory."}
                </p>
            </button>
            <br />
            <div>
                <h2>{"Recent projects"}</h2>
                <div>
                    <p class="tertiary">{"todo"}</p>
                </div>
            </div>
        </main>
    }
}
