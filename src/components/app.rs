use yew::prelude::*;

#[function_component]
pub(crate) fn App() -> Html {
    html! {<main>{"home"}</main>}
}

// fn html_home() -> Html {
//     html! {<span>{"home"}</span>}
// }
//
// fn html_state_query(error_msg_opt: Option<&str>) -> Html {
//     html! {
//         <span style="height: 100%; display: flex; flex-direction: column;">
//             <main class="grid-list-center" style="flex: 1 1 0%;">
//                 {if let Some(error_msg) = error_msg_opt {
//                     html_backend_error(AttrValue::Rc(error_msg.into()))
//                 }
//                 else {
//                     html_waiting_for_backend()
//                 }}
//             </main>
//             <footer>
//                 <h2 class="drop-shadow text-tertiary font-sans" style="text-align: center;">
//                     {format!("chipbox {version}", version = env!("CARGO_PKG_VERSION"))}
//                 </h2>
//             </footer>
//         </span>
//     }
// }
//
// fn html_backend_error(error_msg: AttrValue) -> Html {
//     html! {
//         <Card title="Backend error" msg={error_msg} card_type={CardType::Error}/>
//     }
// }
//
// fn html_waiting_for_backend() -> Html {
//     html! {
//         <span class="grid-list-center">
//             <Spinner class="drop-shadow"/>
//             <h1 class="drop-shadow text-primary font-sans">
//                 {"Waiting for backend."}
//             </h1>
//         </span>
//     }
// }
