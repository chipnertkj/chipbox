use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq + yew::ToHtml + std::fmt::Display + 'static,
{
    pub tabs: &'static [T],
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub class: AttrValue,
}

#[function_component]
pub fn Panel<T>(props: &Props<T>) -> Html
where
    T: PartialEq + yew::ToHtml + std::fmt::Display + 'static,
{
    // Retrieve props.
    let Props { style, class, tabs } = props;

    let style = format!("{}", style);
    let class = format!("panel-root {}", class);

    // Set up state.
    let tab_idx = use_state_eq::<usize, _>(|| 0);

    // Ensure tab index is in range.
    if *tab_idx >= tabs.len() && !tabs.is_empty() {
        tab_idx.set(0);
    }

    let tab_onclick = {
        let tab_idx = tab_idx.clone();
        move |_ev: web_sys::MouseEvent, idx: usize| {
            tracing::trace!("Tab index changed to {}.", idx);
            tab_idx.set(idx)
        }
    };

    html! {
        <span style={style} class={class}>
            <span class="panel-header">
                {
                    tabs.iter().enumerate().map(|(idx, tab)| {
                        let tab_onclick = tab_onclick.clone();
                        html! {
                            <button key={idx} class="panel-tab" onclick={move |ev| tab_onclick(ev, idx)}>
                                <p class="panel-tab-title">
                                    {tab.to_string()}
                                </p>
                            </button>
                        }
                    }).collect::<Html>()
                }
            </span>
            <br/>
            <span class="panel-content">
                {tabs[*tab_idx].to_html()}
            </span>
        </span>
    }
}
