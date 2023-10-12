use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    #[prop_or_default]
    pub(super) style: AttrValue,
    #[prop_or_default]
    pub(super) class: AttrValue,
}

#[function_component]
pub(super) fn Timeline(props: &Props) -> yew::Html {
    // Retrieve props.
    let Props { style, class } = props;

    // Debug info.
    tracing::trace!("Rendering Timeline component.");

    html! {
        <div style={style} class={class.to_string()}>
            <div style="">
            </div>
        </div>
    }
}
