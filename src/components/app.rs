mod setup;

use self::setup::Setup;
use chipbox_glue as glue;
use yew::prelude::*;

pub(crate) enum Msg {
    Update(App),
}

#[derive(Default)]
pub(crate) enum App {
    #[default]
    Querying,
    Error(AttrValue),
    LoadingSettings,
    Setup,
}

impl yew::Component for App {
    type Message = Msg;
    type Properties = ();

    /// Create default `App` and query backend state.
    fn create(ctx: &Context<Self>) -> Self {
        Self::emit_state_query(ctx);
        Default::default()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self {
            Self::Querying => Self::view_querying(),
            Self::Error(e) => Self::view_error(e),
            Self::LoadingSettings => Self::view_loading_settings(),
            Self::Setup => Self::view_setup(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(app) => {
                *self = app;
                true
            }
        }
    }
}

impl App {
    fn emit_state_query(ctx: &Context<Self>) {
        ctx.link()
            .callback_future(|_: ()| async {
                let query = glue::state::query().await;
                let state = query.into();
                Msg::Update(state)
            })
            .emit(());
    }

    fn view_querying() -> Html {
        html! {
            <h1>{"waiting for app state"}</h1>
        }
    }

    fn view_error(e: &AttrValue) -> Html {
        html! {
            <main>
                <h1>{"encountered an error while querying app state: "}</h1>
                <ul>
                    {
                        e.split(": ").enumerate().map(|(n, x)| html! {
                            <li key={n}>{format!("{n}: {x}")}</li>
                        }).collect::<Html>()
                    }
                </ul>
            </main>
        }
    }

    fn view_loading_settings() -> Html {
        html! {
            <h1>{"loading settings"}</h1>
        }
    }

    fn view_setup() -> Html {
        html! {
            <Setup/>
        }
    }
}

/// Convert backend state query into frontend state.
impl From<glue::state::Query> for App {
    fn from(query: glue::state::Query) -> Self {
        match query.result {
            Ok(state) => state.into(),
            Err(e) => Self::Error(e),
        }
    }
}

/// Convert backend state into frontend state.
impl From<glue::State> for App {
    fn from(state: glue::State) -> Self {
        match state {
            glue::State::LoadingSettings => Self::LoadingSettings,
            glue::State::Setup => Self::Setup,
            _ => unimplemented!(),
        }
    }
}
