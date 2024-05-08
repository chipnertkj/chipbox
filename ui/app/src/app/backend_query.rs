use chipbox_ui_spinner::Spinner;
use yew::prelude::*;

#[derive(PartialEq, Default, Clone, Copy)]
pub(super) enum BackendQueryState {
    #[default]
    WaitForBackend,
    ReadSettings,
    QuerySettings,
}

impl AsRef<str> for BackendQueryState {
    fn as_ref(&self) -> &str {
        match self {
            Self::WaitForBackend => "Waiting for backend...",
            Self::ReadSettings => "Reading settings...",
            Self::QuerySettings => "Querying settings...",
        }
    }
}

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub(super) state: BackendQueryState,
}

#[function_component]
pub(super) fn BackendQuery(props: &Props) -> yew::Html {
    const ROOT_STYLE: &str =
        "height: 100vh; display: flex; justify-content: center; \
        flex-direction: column; text-align: center;";
    const MAIN_STYLE: &str = const_format::formatc!("flex: 1; {}", ROOT_STYLE);
    const FOOTER_SYLE: &str =
        const_format::formatc!("flex: 0; margin-bottom: 1rem; {}", ROOT_STYLE);
    const FOOTER_TEXT: &str =
        const_format::formatc!("chipbox {}", env!("CARGO_PKG_VERSION"));
    let message_text = props.state.as_ref();

    html! {
        <div style={ROOT_STYLE}>
            <main style={MAIN_STYLE}>
                <Spinner container_class="drop-shadow" svg_class="primary" />
                <h1 class="primary header drop-shadow">
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
