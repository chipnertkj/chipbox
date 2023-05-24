#![deny(unsafe_op_in_unsafe_fn)]

mod app;
mod renderer;
mod ui;

use tracing_subscriber::util::SubscriberInitExt as _;
use winit::event_loop;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // This value will never be dropped due to `app::App::run` taking over the thread.
    // This is, as far as I'm aware, perfectly ok.
    tracing_subscriber::FmtSubscriber::default().init();

    let event_loop = event_loop::EventLoop::default();
    app::App::new(&event_loop)
        .await
        .run(event_loop)
        .await;
}
