use cowstr::CowStr;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub(super) enum SetupState {
    First,
    Error(CowStr),
}

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: SetupState,
}

#[function_component]
pub(super) fn Setup(props: &Props) -> yew::Html {
    match props.state {
        SetupState::First => html_first(),
        SetupState::Error(ref _err) => todo!(),
    }
}

fn html_first() -> yew::Html {
    let on_click = move |_: MouseEvent| {};

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
