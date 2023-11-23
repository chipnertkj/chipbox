use chipbox_ui_panel::Panel;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    #[prop_or_default]
    pub(super) style: AttrValue,
    #[prop_or_default]
    pub(super) class: AttrValue,
}

#[derive(PartialEq)]
pub enum Tab {
    Pattern,
    NodeTree,
    Audio,
}

impl yew::ToHtml for Tab {
    fn to_html(&self) -> yew::virtual_dom::VNode {
        match self {
            Self::Pattern => html! { "Pattern" },
            Self::NodeTree => html! { "Node Tree" },
            Self::Audio => html! { "Audio" },
        }
    }
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pattern => write!(f, "Pattern"),
            Self::NodeTree => write!(f, "Node Tree"),
            Self::Audio => write!(f, "Audio"),
        }
    }
}

#[function_component]
pub(super) fn MainPanel(props: &Props) -> yew::Html {
    // Retrieve props.
    let Props { style, class } = props;

    let style = format!("{}", style);
    let class = format!("{}", class);

    const TABS: &[Tab] = &[Tab::Pattern, Tab::NodeTree, Tab::Audio];

    html! {
        <Panel<Tab> id="main-panel" style={style} class={class} tabs={TABS}/>
    }
}
