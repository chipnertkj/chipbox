use chipbox_glue as glue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct SetupProps {
    pub state: glue::app::Setup,
}

#[function_component]
pub(super) fn Setup(props: &SetupProps) -> yew::Html {
    use glue::app::Setup;

    let SetupProps { state } = props;

    match state {
        Setup::First => html! {
            <>
                <h1>{"First time setup"}</h1>
                <button>
                    <h2>{"Skip setup"}</h2>
                    <h3>{"Use default settings"}</h3>
                </button>
            </>
        },
        Setup::Error(error) => html! {
            <>
                <h1>{"Error reading settings"}</h1>
                <code>{error}</code>
            </>
        },
        Setup::Modify(settings) => html! {
            <>
                <h1>{"Configure settings"}</h1>
                <h2>{format!("{:?}", *settings)}</h2>
            </>
        },
    }
}
