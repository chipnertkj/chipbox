use yew::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub(super) enum State {
    First,
}

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: State,
}

#[function_component]
pub(super) fn Setup(props: &Props) -> yew::Html {
    match props.state {
        State::First => html_first(),
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
