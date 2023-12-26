use bottom_panel::BottomPanel;
use main_panel::MainPanel;
use right_panel::RightPanel;
use yew::prelude::*;

mod bottom_panel;
mod main_panel;
mod right_panel;

#[derive(Properties, PartialEq)]
pub(super) struct Props {}

#[function_component]
pub(super) fn Editor(props: &Props) -> yew::Html {
    // Main styling.
    const MAIN_STYLE: &str = "width: 100vw; height: 100vh;";
    const MAIN_CLASS: &str = "";

    html! {
        <main id={"editor"} style={MAIN_STYLE} class={MAIN_CLASS}>
            <div id={"main-row"}>
                <MainPanel/>
                <RightPanel/>
            </div>
            <BottomPanel/>
        </main>
    }
}
