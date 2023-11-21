use web_sys::wasm_bindgen::JsCast as _;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq + 'static,
{
    pub tabs: &'static [T],
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub class: AttrValue,
    pub id: &'static str,
}

fn tab_elements<T>(id: &str, tabs: &[T]) -> Vec<web_sys::Element> {
    tabs.iter()
        .enumerate()
        .map(|(idx, _tab)| {
            let panel = gloo::utils::document()
                .get_element_by_id(id)
                .expect("panel not found");
            panel
                .query_selector_all(&format!(
                    ".panel-tab:nth-child({})",
                    idx + 1
                ))
                .expect("selector string is invalid")
                .get(0)
                .expect("tab not found")
                .dyn_into::<web_sys::Element>()
                .expect("tab is not an element")
        })
        .collect()
}

fn apply_tab_style(tab_nodes: &[web_sys::Element], selected_idx: usize) {
    tab_nodes
        .iter()
        .enumerate()
        .for_each(|(idx, tab)| {
            const ACTIVE_CLASS: &str = "active";
            const INACTIVE_CLASS: &str = "inactive";
            let (add, remove) = if idx == selected_idx {
                (ACTIVE_CLASS, INACTIVE_CLASS)
            } else {
                (INACTIVE_CLASS, ACTIVE_CLASS)
            };
            tab.class_list()
                .remove_1(remove)
                .unwrap();
            tab.class_list()
                .add_1(add)
                .unwrap();
        })
}

fn html_tabs<T, F>(tabs: &[T], tab_onclick: F) -> Html
where
    F: Fn(MouseEvent, usize) + Clone + 'static,
{
    tabs.iter().enumerate().map(|(idx, _tab)| {
        let tab_onclick = tab_onclick.clone();
        html! {
            <button key={idx} class="panel-tab inactive" onclick={move |ev| tab_onclick(ev, idx)}>
                <p class="panel-tab-title">
                    {idx}
                </p>
            </button>
        }
    }).collect::<Html>()
}

#[function_component]
pub fn Panel<T>(props: &Props<T>) -> Html
where
    T: PartialEq + yew::ToHtml + std::fmt::Display + 'static,
{
    let style = format!("{}", props.style);
    let class = format!("panel-root {}", props.class);
    let tabs = props.tabs;
    let id = props.id;

    // Set up state.
    let tab_idx = use_state_eq::<usize, _>(|| 0);

    // Ensure tab index is in range.
    if *tab_idx >= tabs.len() && !tabs.is_empty() {
        tab_idx.set(0);
    }

    // Set up tab index change handler.
    let tab_onclick = {
        let tab_idx = tab_idx.clone();
        move |_ev: web_sys::MouseEvent, idx: usize| {
            tracing::trace!("Tab index changed to {} ({}).", idx, tabs[idx]);
            tab_idx.set(idx)
        }
    };

    // Set up tab style effect hook.
    yew::use_effect({
        let tab_idx = tab_idx.clone();
        move || {
            let tab_elements = tab_elements(id, tabs);
            apply_tab_style(&tab_elements, *tab_idx);
        }
    });

    html! {
        <div id={id} style={style} class={class}>
            <div class="panel-header">
                { html_tabs(tabs, tab_onclick) }
            </div>
            <div class="panel-content">
                {tabs[*tab_idx].to_html()}
            </div>
        </div>
    }
}
