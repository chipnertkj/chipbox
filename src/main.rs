use tracing_subscriber::util::SubscriberInitExt as _;
use winit::event_loop;

mod application;
mod renderer;
mod settings;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder().finish();
    subscriber.init();

    let event_loop = event_loop::EventLoop::default();

    let chipbox = application::Chipbox::new(&event_loop);
    chipbox.run(event_loop);
}
