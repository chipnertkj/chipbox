use tracing_subscriber::util::SubscriberInitExt as _;
use winit::event_loop;

mod chipbox;

fn main() {
    todo!("remove (refactor) module chipbox. redundant: `chipbox::chipbox`");
    todo!("rename settings to config");
    let subscriber = tracing_subscriber::FmtSubscriber::builder().finish();
    subscriber.init();

    let event_loop = event_loop::EventLoop::default();

    let chipbox = chipbox::Chipbox::new(&event_loop);
    chipbox.run(event_loop);
}
