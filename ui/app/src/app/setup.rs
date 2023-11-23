use super::{set_default_ctx_settings, AppContext, RerenderCallback};
use yew::platform::spawn_local;
use yew::prelude::*;
use {chipbox_common as common, chipbox_glue as glue};

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub state: glue::Setup,
}

#[function_component]
pub(super) fn Setup(props: &Props) -> yew::Html {
    // Retrieve state.
    let Props { state } = props;

    // Acquire app context.
    let app_ctx = use_context::<AppContext>()
        // App context should be available at this point.
        .expect("no app context");

    // Update context settings.
    set_default_ctx_settings(app_ctx.clone());

    match state {
        glue::Setup::First => html_first(app_ctx.rerender_cb),
        glue::Setup::Error(error) => html_error(error),
        glue::Setup::Modify(settings) => html_modify(settings),
    }
}

fn html_first(rerender_cb: RerenderCallback) -> yew::Html {
    let on_click = move |_: MouseEvent| {
        let rerender_cb = rerender_cb.clone();
        spawn_local(async move {
            let response = glue::skip_setup::query().await;
            if let Ok(()) = response {
                rerender_cb.emit();
            }
        });
    };

    html! {
        <>
            <h1>{"First time setup"}</h1>
            <button onclick={on_click}>
                <h2 class="left">{"Skip setup"}</h2>
                <p class="tertiary left">{"Use default settings"}</p>
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
