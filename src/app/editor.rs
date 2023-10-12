mod timeline;

use crate::app::{update_ctx_settings, AppContext};
use chipbox_glue as glue;
use timeline::Timeline;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: glue::app::Editor,
}

#[function_component]
pub(super) fn Editor(props: &Props) -> yew::Html {
    // Debug info.
    tracing::trace!("Rendering Editor component.");

    // Retrieve state.
    let Props { state } = props;

    // Acquire app context.
    let mut app_ctx = use_context::<AppContext>().expect("no app context");

    // Update context settings.
    update_ctx_settings(state, &mut app_ctx);

    // Main styling.
    const MAIN_STYLE: &str = "width: 100vw; height: 100vh;";
    const MAIN_CLASS: &str = "";
    // Timeline styling.
    const TIMELINE_STYLE: &str = "width: 100%; height: 100%; overflow: auto;";
    const TIMELINE_CLASS: &str = "";

    html! {
        <main style={MAIN_STYLE} class={MAIN_CLASS}>
            <Timeline style={TIMELINE_STYLE} class={TIMELINE_CLASS}/>
        </main>
    }
}
