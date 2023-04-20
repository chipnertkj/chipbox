use tracing_subscriber::util::SubscriberInitExt as _;
use winit::event_loop;

mod application;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::default();
    subscriber.init();

    let event_loop = event_loop::EventLoop::default();
    application::Chipbox::new(&event_loop).run(event_loop)
}
