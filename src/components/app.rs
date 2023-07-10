use crate::commands;
use crate::components::*;
use chipbox_common::{AppState, SettingsLoadError};
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let state_result_opt = use_state(|| None);

    // Query state on init.
    use_memo(
        |_| {
            let state_result_opt = state_result_opt.clone();
            spawn_local(async move {
                state_result_opt.set(Some(commands::query_app_state().await));
            })
        },
        (),
    );

    match *state_result_opt {
        None => html_state_query(None),
        Some(ref state_result) => match state_result
            .as_ref()
            .expect("should be `!`")
        {
            AppState::Setup { settings_result } => match settings_result {
                Ok(_) => html_home(),
                Err(SettingsLoadError::NotFound) => html_home(),
                Err(e) => html_state_query(Some(e.to_string().as_str())),
            },
            _ => unreachable!(),
        },
    }
}

fn html_home() -> Html {
    html! {<span>{"home"}</span>}
}

fn html_state_query(error_msg_opt: Option<&str>) -> Html {
    html! {
        <span style="height: 100%; display: flex; flex-direction: column;">
            <main class="grid-list-center" style="flex: 1 1 0%;">
                {if let Some(error_msg) = error_msg_opt {
                    html_backend_error(AttrValue::Rc(error_msg.into()))
                }
                else {
                    html_waiting_for_backend()
                }}
            </main>
            <footer>
                <h2 class="drop-shadow text-tertiary font-sans" style="text-align: center;">
                    {format!("chipbox {version}", version = env!("CARGO_PKG_VERSION"))}
                </h2>
            </footer>
        </span>
    }
}

fn html_backend_error(error_msg: AttrValue) -> Html {
    html! {
        <Card title="Backend error" msg={error_msg} card_type={CardType::Error}/>
    }
}

fn html_waiting_for_backend() -> Html {
    html! {
        <span class="grid-list-center">
            <Spinner class="drop-shadow"/>
            <h1 class="drop-shadow text-primary font-sans">
                {"Waiting for backend."}
            </h1>
        </span>
    }
}
