use crate::Tab;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast as _;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub class: AttrValue,
    pub id: &'static str,
}

pub struct Panel<T>
where
    T: Tab<T>,
    [(); T::TABS.len()]:,
{
    tab_idx: Rc<Cell<usize>>,
    msg_cb: Callback<()>,
    _msg_cl: Closure<dyn FnMut()>,
    tab_refs: Rc<[NodeRef; T::TABS.len()]>,
    left_gradient_ref: NodeRef,
    right_gradient_ref: NodeRef,
    overflow_x_ref: NodeRef,
    overflow_x_observer: web_sys::ResizeObserver,
}

impl<T> Component for Panel<T>
where
    T: Tab<T>,
    [(); T::TABS.len()]:,
{
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let tab_idx = Rc::new(Cell::new(0));
        let tab_refs = Rc::new(std::array::from_fn(|_| NodeRef::default()));
        let left_gradient_ref = NodeRef::default();
        let right_gradient_ref = NodeRef::default();
        let overflow_x_ref = NodeRef::default();
        let msg_cb = ctx
            .link()
            .callback(|_: ()| ());

        let msg_cl = {
            let msg_cb = msg_cb.clone();
            wasm_bindgen::closure::Closure::new(move || msg_cb.emit(()))
        };
        let msg_fn = msg_cl
            .as_ref()
            .unchecked_ref();

        let overflow_x_observer = web_sys::ResizeObserver::new(msg_fn)
            .expect("failed to create observer");

        Self {
            tab_idx,
            msg_cb,
            _msg_cl: msg_cl,
            tab_refs,
            left_gradient_ref,
            right_gradient_ref,
            overflow_x_ref,
            overflow_x_observer,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        self.tab_refs
            .iter()
            .enumerate()
            .for_each(|(idx, node_ref)| self.apply_tab_style(idx, node_ref));
        let overflow_x = self
            .overflow_x_ref
            .cast::<web_sys::Element>()
            .expect("should be element");
        self.overflow_x_observer
            .observe(&overflow_x);
        Self::apply_gradient_style(
            &self.left_gradient_ref,
            &self.right_gradient_ref,
            overflow_x,
        )
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = &ctx.props();
        let style = format!("{}", props.style);
        let class = format!("panel-root {}", props.class);
        let id = props.id;

        let onscroll = {
            let left_gradient_ref = self.left_gradient_ref.clone();
            let right_gradient_ref = self
                .right_gradient_ref
                .clone();
            Callback::from(move |event: Event| {
                let target: web_sys::Element = event
                    .target()
                    .expect("should have target")
                    .dyn_into()
                    .expect("should be element");
                Self::apply_gradient_style(
                    &left_gradient_ref,
                    &right_gradient_ref,
                    target,
                );
            })
        };

        html! {
            <div id={id} style={style} class={class}>
                <div class="panel-container">
                    <div class="panel-header">
                        <div class="panel-header-overflow-clip">
                            <div ref={self.overflow_x_ref.clone()} onscroll={onscroll} class="panel-header-overflow-x">
                                {
                                    T::TABS
                                        .iter()
                                        .enumerate()
                                        .map(|(idx, tab)| {
                                            self.html_tab(idx, tab)
                                        }).collect::<Html>()
                                }
                            </div>
                        </div>
                        <div ref={self.left_gradient_ref.clone()} class="panel-header-gradient left" />
                        <div ref={self.right_gradient_ref.clone()} class="panel-header-gradient right" />
                    </div>
                    <div class="panel-content">
                        {T::TABS[self.tab_idx.get()].to_html()}
                    </div>
                </div>
            </div>
        }
    }
}

impl<T> Panel<T>
where
    T: Tab<T>,
    [(); T::TABS.len()]:,
{
    fn apply_tab_style(&self, idx: usize, node_ref: &NodeRef) {
        let is_active = self.tab_idx.get() == idx;
        let class_list = node_ref
            .cast::<web_sys::Element>()
            .expect("tab ref should be element")
            .class_list();
        const ACTIVE_CLASS: &str = "active";
        const INACTIVE_CLASS: &str = "inactive";
        let (add, remove) = if is_active {
            (ACTIVE_CLASS, INACTIVE_CLASS)
        } else {
            (INACTIVE_CLASS, ACTIVE_CLASS)
        };
        class_list
            .add_1(add)
            .expect("should be able to add class");
        class_list
            .remove_1(remove)
            .expect("should be able to remove class");
    }

    fn html_tab(&self, idx: usize, tab: &T) -> Html {
        let msg_cb = self.msg_cb.clone();
        let tab_idx = self.tab_idx.clone();
        let is_active = self.tab_idx.get() == idx;
        html! {
            <button
                ref={self.tab_refs[idx].clone()}
                key={idx}
                class="panel-tab"
                onclick={
                    move |_| {
                        if !is_active {
                            tab_idx.set(idx);
                            msg_cb.emit(());
                        }
                    }
                }
            >
                <p class="panel-tab-title">{tab.to_string()}</p>
            </button>
        }
    }

    fn apply_gradient_style(
        left_gradient_ref: &NodeRef,
        right_gradient_ref: &NodeRef,
        overflow_x: web_sys::Element,
    ) {
        let left_gradient = left_gradient_ref
            .cast::<web_sys::Element>()
            .expect("should be element");
        let right_gradient = right_gradient_ref
            .cast::<web_sys::Element>()
            .expect("should be element");

        let scroll_left = overflow_x.scroll_left();
        let client_width = overflow_x.client_width();
        let scroll_width = overflow_x.scroll_width();

        const ACTIVE_CLASS: &str = "active";

        if scroll_left == 0 {
            left_gradient
                .class_list()
                .remove_1(ACTIVE_CLASS)
                .expect("failed to remove class");
        } else {
            left_gradient
                .class_list()
                .add_1(ACTIVE_CLASS)
                .expect("failed to add class");
        }

        if scroll_left + client_width >= scroll_width {
            right_gradient
                .class_list()
                .remove_1(ACTIVE_CLASS)
                .expect("failed to remove class");
        } else {
            right_gradient
                .class_list()
                .add_1(ACTIVE_CLASS)
                .expect("failed to add class");
        }
    }
}
