use yew::prelude::*;

use crate::util;

pub(crate) enum Setup {
    Welcome { after_config: bool },
    Config,
    Demo,
}

impl Default for Setup {
    fn default() -> Self {
        Setup::Welcome {
            after_config: false,
        }
    }
}

pub(crate) enum Msg {
    Update(Setup),
}

impl Component for Setup {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Setup::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self {
            Setup::Welcome { after_config } => {
                Setup::view_welcome(ctx, *after_config)
            }
            Setup::Config => Setup::view_config(),
            Setup::Demo => Setup::view_demo(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(setup) => {
                *self = setup;
                true
            }
        }
    }
}

impl Setup {
    fn view_welcome(ctx: &Context<Self>, after_config: bool) -> Html {
        let onclick = ctx
            .link()
            .callback(|_| Msg::Update(Setup::Config));
        html! {
            <main>
                <h1>{"First-time setup"}</h1>
                <h2>{"Welcome to chipbox."}</h2>
                <p>{"What would you like to do?"}</p>
                <button {onclick}>{"Configure chipbox"}</button>
                <button>{"See a demo showcase"}</button>
            </main>
        }
    }

    fn view_config() -> Html {
        util::enable_confirm_refresh();
        html! {
            <main>
                <h1>{"Configure chipbox"}</h1>
                <h2>{"Select the configuration process you'd like to use."}</h2>
                <button>
                    <p>{"Use default settings"}</p>
                    <p>{"Skip the first-time setup and use pre-defined settings."}</p>
                </button>
                <button>
                    <p>{"Full setup"}</p>
                    <p>{"Configure all settings manually."}</p>
                </button>
            </main>
        }
    }

    fn view_demo() -> Html {
        html! {}
    }
}
