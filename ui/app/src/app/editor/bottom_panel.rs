use chipbox_ui_panel::Panel;
use yew::prelude::*;

mod timeline;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    #[prop_or_default]
    pub(super) style: AttrValue,
    #[prop_or_default]
    pub(super) class: AttrValue,
}

#[derive(PartialEq)]
pub enum Tab {
    Timeline,
    Mixer,
}

impl yew::ToHtml for Tab {
    fn to_html(&self) -> yew::virtual_dom::VNode {
        match self {
            Self::Timeline => html! { "Timeline" },
            Self::Mixer => html! { "Mixer" },
        }
    }
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeline => write!(f, "Timeline"),
            Self::Mixer => write!(f, "Mixer"),
        }
    }
}

#[function_component]
pub(super) fn BottomPanel(props: &Props) -> yew::Html {
    // Retrieve props.
    let Props { style, class } = props;

    let style = format!("{}", style);
    let class = format!("{}", class);

    const TABS: &[Tab] = &[Tab::Timeline, Tab::Mixer];

    html! {
        <Panel<Tab> style={style} class={class} tabs={TABS}>
        </Panel<Tab>>
    }
}
