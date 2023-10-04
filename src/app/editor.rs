use crate::app::{update_ctx_settings, AppContext};
use chipbox_glue as glue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: glue::app::Editor,
}

#[function_component]
pub(super) fn Editor(props: &Props) -> yew::Html {
    // Debug info.
    tracing::trace!("Rendering component.");

    // Retrieve state.
    let Props { state } = props;

    // Acquire app context.
    let mut app_ctx = use_context::<AppContext>().expect("no app context");

    // Update context settings.
    update_ctx_settings(state, &mut app_ctx);

    html! {
        <main>

        </main>
    }
}
