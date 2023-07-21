use yew::prelude::*;

#[derive(PartialEq)]
pub(crate) enum Type {
    Error,
}

#[derive(PartialEq, Properties)]
pub(crate) struct CardProps {
    pub(crate) title: AttrValue,
    pub(crate) msg: AttrValue,
    pub(crate) card_type: Type,
}

#[function_component]
pub(crate) fn Card(props: &CardProps) -> Html {
    let CardProps {
        title,
        msg,
        card_type,
    } = props;
    html! {
        <span class="card">
            <h2 class="card-title">
                {title}
            </h2>
            <span class="card-msg">
                <h1 class="card-msg-text">
                    {msg}
                </h1>
            </span>
        </span>
    }
}
