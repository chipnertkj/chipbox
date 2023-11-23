use super::{set_default_ctx_settings, AppContext};
use chipbox_glue as glue;
use chipbox_ui_spinner::Spinner;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: glue::app::QueryingBackend,
}

#[function_component]
pub(super) fn QueryingBackend(props: &Props) -> yew::Html {
    // Retrieve state.
    let Props { state } = props;

    // Acquire app context.
    let app_ctx = use_context::<AppContext>()
        // App context should be available at this point.
        .expect("no app context");

    // Update context settings.
    set_default_ctx_settings(app_ctx);

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
        glue::app::QueryingBackend::TimedOut(_) => {
            "Timed out while waiting for state."
        }
    };

    html! {
        <div style={ROOT_STYLE}>
            <main style={MAIN_STYLE}>
                if let glue::app::QueryingBackend::TimedOut(_) = state {
                    <h1 class="drop-shadow primary">
                        {":("}
                    </h1>
                }
                else {
                    <Spinner class="drop-shadow secondary" svg_class="primary" />
                }
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
