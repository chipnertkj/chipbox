use {chipbox_common as common, chipbox_glue as glue};

use editor::Editor;
use home::Home;
use querying_backend::QueryingBackend;
use setup::Setup;
use yew::platform::spawn_local;
use yew::prelude::*;

mod editor;
mod home;
mod querying_backend;
mod setup;

#[derive(PartialEq, Default)]
enum RenderState {
    #[default]
    Requested,
    Idle,
}

#[derive(Clone, PartialEq)]
pub struct RerenderCallback {
    pub inner: Callback<()>,
}

#[derive(Clone, PartialEq)]
pub struct AppContext {
    pub rerender_cb: RerenderCallback,
    pub settings: common::Settings,
}

#[function_component]
pub fn App() -> yew::Html {
    // Set up state.
    let app_state = use_state(glue::App::default);
    let render_state = use_state(RenderState::default);

    // Set up app context.
    let app_ctx = {
        let render_state = render_state.clone();
        AppContext {
            rerender_cb: RerenderCallback {
                inner: Callback::from(move |_: ()| {
                    render_state.set(RenderState::Requested);
                }),
            },
            settings: Default::default(),
        }
    };

    // Rerender page on RenderState::Requested.
    use_memo(render_state, |render_state| {
        handle_rerender(app_state.clone(), render_state.clone())
    });

    // Render main page content.
    let content = match &*app_state {
        glue::App::QueryingBackend(state) => html! {
            <QueryingBackend state={*state} />
        },
        glue::App::Setup(state) => html! {
            <Setup state={state.clone()} />
        },
        glue::App::Home(state) => html! {
            <Home state={state.clone()} />
        },
        glue::App::Editor(state) => html! {
            <Editor state={state.clone()} />
        },
    };

    // Wrap page content in a ContextProvider.
    html! {
        // We're using the alternative children syntax due to a bug in
        // HTML syntax highlighting extensions.
        <ContextProvider<AppContext> context={app_ctx} children={content}/>
    }
}

// Rerender page on RenderState::Requested.
fn handle_rerender(
    app_state: yew::UseStateHandle<glue::App>,
    render_state: yew::UseStateHandle<RenderState>,
) {
    // Ignore on RenderState::Idle.
    if *render_state == RenderState::Requested {
        tracing::trace!("Manual rerender.");
        spawn_local(async move {
            // Update app state.
            let response = glue::app::query().await;
            match response {
                Ok(app) => {
                    app_state.set(app);
                }
                Err(e) => {
                    app_state.set(glue::App::QueryingBackend(
                        glue::app::QueryingBackend::TimedOut(e),
                    ));
                }
            }
            // Reset render state.
            render_state.set(RenderState::Idle);
        })
    }
}

pub fn update_ctx_settings(
    state: &impl glue::ConfiguredState,
    app_ctx: &mut AppContext,
) {
    // Retrieve settings.
    let settings = state
        .settings()
        .as_ref()
        .to_owned();
    // Update settings in app context.
    tracing::trace!(
        "Updating app context with settings from configured state: {:?}",
        settings
    );
    app_ctx.settings = settings;
    // Apply settings to page.
    apply_settings(app_ctx);
}

pub fn set_default_ctx_settings(app_ctx: &mut AppContext) {
    // Retrieve default settings.
    let settings = Default::default();
    // Update settings in app context.
    tracing::trace!(
        "Updating app context with default settings: {:?}",
        settings
    );
    app_ctx.settings = settings;
    // Apply settings to page.
    apply_settings(app_ctx);
}

fn apply_settings(app_ctx: &AppContext) {
    // Retrieve settings.
    let settings = &app_ctx.settings;

    tracing::trace!("Applying settings: {settings:?}");

    // Retrieve theme.
    let theme = settings
        .user_themes
        .theme(&settings.selected_theme);
    // Apply theme.
    match theme {
        Option::Some(theme) => apply_theme(theme),
        Option::None => {
            tracing::info!(
                "Updating backend settings due to invalid theme selection."
            );
            update_backend_settings(settings.clone());
        }
    }
}

fn update_backend_settings(settings: common::Settings) {
    spawn_local(async {
        let result = glue::set_settings::query(settings).await;
        tracing::info!("Backend settings update result: {result:?}");
    })
}

fn apply_theme(theme: &common::Theme) {
    tracing::trace!("Applying theme: {theme:?}");
}

fn rerender(app_ctx: AppContext) {
    app_ctx
        .rerender_cb
        .inner
        .emit(());
}
