use chipbox_glue as glue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: glue::app::QueryingBackend,
}

#[function_component]
pub(super) fn QueryingBackend(props: &Props) -> yew::Html {
    let Props { state } = props;

    match state {
        glue::app::QueryingBackend::Requesting => html! {
            <h1>{"Requesting state"}</h1>
        },
        glue::app::QueryingBackend::ReadingSettings => html! {
            <h1>{"Reading Settings"}</h1>
        },
    }
}
