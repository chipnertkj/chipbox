use crate::app::RerenderCallback;
use chipbox_glue as glue;
use const_format::formatc;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: glue::app::Home,
}

#[function_component]
pub(super) fn Home(props: &Props) -> yew::Html {
    let Props { state } = props;

    let rerender_cb = use_context::<RerenderCallback>()
        .expect("no rerender callback context")
        .inner;

    let on_click_new = move |_| {
        let rerender_cb = rerender_cb.clone();
        spawn_local(async move {
            let info = glue::LoadProjectInfo::New;
            let response = glue::load_project::query(info).await;
            tracing::info!("response: {:?}", response);
            rerender_cb.emit(());
        });
    };

    html! {
        <main>
            <h1 class="title">
                {"chipbox"}
                <code class="header tertiary code">{formatc!("v{}", env!("CARGO_PKG_VERSION"))}</code>
            </h1>
            <button onclick={on_click_new}>
                <h2 class="left">{"Create a new project"}</h2>
                <p class="tertiary left">{"Continue to the editor."}</p>
            </button>
            <button>
                <h2 class="left">{"User projects"}</h2>
                <p class="tertiary left">{"Browse projects in the user directory."}</p>
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
