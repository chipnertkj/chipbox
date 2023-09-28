use yew::platform::spawn_local;
use yew::prelude::*;

use crate::app::RerenderCallback;
use {chipbox_common as common, chipbox_glue as glue};

#[derive(Properties, PartialEq)]
pub(super) struct SetupProps {
    pub state: glue::app::Setup,
}

#[function_component]
pub(super) fn Setup(props: &SetupProps) -> yew::Html {
    use glue::app::Setup;
    let SetupProps { state } = props;

    let rerender_cb = use_context::<RerenderCallback>()
        .expect("no rerender callback context")
        .inner;

    match state {
        Setup::First => html_first(rerender_cb),
        Setup::Error(error) => html_error(error),
        Setup::Modify(settings) => html_modify(settings),
    }
}

fn html_first(rerender_cb: yew::Callback<()>) -> yew::Html {
    let on_click = move |_: MouseEvent| {
        let rerender_cb = rerender_cb.clone();
        spawn_local(async move {
            let response = glue::skip_setup::query().await;
            if let Ok(()) = response {
                rerender_cb.emit(());
            }
        });
    };

    html! {
        <>
            <h1>{"First time setup"}</h1>
            <button onclick={on_click}>
                <h2>{"Skip setup"}</h2>
                <h3>{"Use default settings"}</h3>
            </button>
        </>
    }
}

fn html_error(error: &str) -> yew::Html {
    html! {
        <>
            <h1>{"Error reading settings"}</h1>
            <code>{error}</code>
        </>
    }
}

fn html_modify(settings: &common::Settings) -> yew::Html {
    html! {
        <>
            <h1>{"Configure settings"}</h1>
            <h2>{format!("{:?}", *settings)}</h2>
        </>
    }
}
