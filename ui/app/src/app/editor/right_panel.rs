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
    Slot,
    Project,
}

impl chipbox_ui_panel::Tab<Self> for Tab {
    const TABS: &'static [Self] = &[Self::Slot, Self::Project];
}

impl yew::ToHtml for Tab {
    fn to_html(&self) -> yew::virtual_dom::VNode {
        match self {
            Self::Slot => html! { "Slot" },
            Self::Project => html! { "Project" },
        }
    }
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Slot => write!(f, "Slot"),
            Self::Project => write!(f, "Project"),
        }
    }
}

#[function_component]
pub(super) fn RightPanel(props: &Props) -> yew::Html {
    // Retrieve props.
    let Props { style, class } = props;

    let style = format!("{}", style);
    let class = format!("{}", class);

    html! {
        <Panel<Tab> id="right-panel" style={style} class={class}/>
    }
}
