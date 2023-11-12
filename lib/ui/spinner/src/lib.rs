use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: AttrValue,
    pub svg_class: AttrValue,
}

#[function_component]
pub fn Spinner(props: &Props) -> yew::Html {
    let Props { class, svg_class } = props;
    html! {
        <span class={format!("{class}")}>
            <svg class={format!("spinner {svg_class}")} viewBox="0 0 50 50">
                <circle class="path" cx="25" cy="25" r="20" fill="none" stroke-width="5"></circle>
            </svg>
        </span>
    }
}
