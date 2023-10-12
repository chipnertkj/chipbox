use crate::app::{set_default_ctx_settings, AppContext};
use crate::Spinner;
use chipbox_glue as glue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: glue::app::QueryingBackend,
}

#[function_component]
pub(super) fn QueryingBackend(props: &Props) -> yew::Html {
    // Debug info.
    tracing::trace!("Rendering QueryingBackend component.");

    // Retrieve state.
    let Props { state } = props;

    // Acquire app context.
    let mut app_ctx = use_context::<AppContext>().expect("no app context");

    // Update context settings.
    set_default_ctx_settings(&mut app_ctx);

    const ROOT_STYLE: &str =
        "height: 100vh; display: flex; justify-content: center; \
        flex-direction: column; text-align: center;";
    const MAIN_STYLE: &str = const_format::formatc!("flex: 1; {}", ROOT_STYLE);
    const FOOTER_SYLE: &str =
        const_format::formatc!("flex: 0; margin-bottom: 1rem; {}", ROOT_STYLE);
    const FOOTER_TEXT: &str =
        const_format::formatc!("chipbox {}", env!("CARGO_PKG_VERSION"));
    let message_text = match state {
        glue::app::QueryingBackend::Requesting => "Requesting backend state.",
        glue::app::QueryingBackend::ReadingSettings => "Reading user settings.",
    };

    html! {
        <div style={ROOT_STYLE}>
            <main style={MAIN_STYLE}>
                <Spinner class="drop-shadow secondary" svg_class="primary" />
                <h1 class="text drop-shadow secondary">
                    {message_text}
                </h1>
            </main>
            <footer style={FOOTER_SYLE}>
                <h2 class="text drop-shadow tertiary">
                    {FOOTER_TEXT}
                </h2>
            </footer>
        </div>
    }
}
