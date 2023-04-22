use tracing_subscriber::util::SubscriberInitExt as _;
use winit::event_loop;

mod application;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::default();
    subscriber.init();

    let event_loop = event_loop::EventLoop::default();
    let chipbox = application::Chipbox::load_from_config(&event_loop);
    chipbox.run(event_loop)
}
