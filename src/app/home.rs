use chipbox_glue as glue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: glue::app::Home,
}

#[function_component]
pub(super) fn Home(props: &Props) -> yew::Html {
    let Props { state } = props;

    html! {
        <main>
            <h1>{const_format::formatc!("chipbox {}", env!("CARGO_PKG_VERSION"))}</h1>
            <button>
                <h1 class="primary left">{"Create a new project"}</h1>
                <h2 class="secondary left">{"Continue to the editor."}</h2>
            </button>
            <button>
                <h1 class="primary left">{"User projects"}</h1>
                <h2 class="secondary left">{"Browse projects in the user directory."}</h2>
            </button>
            <br />
            <div>
                <h1>{"Recent projects"}</h1>
                <div>
                    <p>{"todo"}</p>
                </div>
            </div>
        </main>
    }
}
