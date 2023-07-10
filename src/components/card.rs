use yew::prelude::*;

#[derive(PartialEq)]
pub enum CardType {
    Error,
}

#[derive(PartialEq, Properties)]
pub struct CardProps {
    pub title: AttrValue,
    pub msg: AttrValue,
    pub card_type: CardType,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
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
