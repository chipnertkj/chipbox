pub(crate) mod tauri_api;

mod app;
mod tracing_layers;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    tracing_layers::init();
    leptos::mount::mount_to_body(|| {
        leptos::view! { <App /> }
    })
}
