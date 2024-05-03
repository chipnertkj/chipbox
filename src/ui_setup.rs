use chipbox_ui_app as ui_app;
use ui_app::App;

pub(super) fn init() {
    let _ = yew::Renderer::<App>::new().render();
}
