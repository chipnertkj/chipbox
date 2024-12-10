use chipbox_glue::cmd;
use leptos::{prelude::*, task};

#[component]
pub(crate) fn Home() -> impl IntoView {
    view! {
        <div id="home">
            <button on:click=move |_| task::spawn_local(
                create_project(),
            )>"Create a new project"</button>
            <button on:click=move |_| task::spawn_local(
                open_project(),
            )>"Open an existing project"</button>
        </div>
    }
}

async fn open_project() {
    todo!()
}

async fn create_project() {
    let cmd_args = todo!();
    let response = cmd::create_project(&cmd_args).await;
}
