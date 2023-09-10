use crate::Spinner;
use chipbox_glue as glue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: glue::app::QueryingBackend,
}

#[function_component]
pub(super) fn QueryingBackend(props: &Props) -> yew::Html {
    let Props { state } = props;

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
        <div style={ROOT_STYLE} class="sans">
            <main style={MAIN_STYLE}>
                <Spinner class="drop-shadow-secondary" svg_class="spinner-primary" />
                <h1 class="text-secondary drop-shadow-secondary">
                    {message_text}
                </h1>
            </main>
            <footer style={FOOTER_SYLE}>
                <h2 class="text-tertiary drop-shadow-tertiary">
                    {FOOTER_TEXT}
                </h2>
            </footer>
        </div>
    }
}
