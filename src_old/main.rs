#![feature(never_type)]
#![feature(future_join)]
#![deny(unsafe_op_in_unsafe_fn)]
// Temporary: remove on release.
#![allow(dead_code)]

use tracing_subscriber::util::SubscriberInitExt as _;
use winit::event_loop;

mod application;
mod config;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::default();
    subscriber.init();

    let event_loop = event_loop::EventLoop::default();
    let chipbox = application::Chipbox::new(&event_loop);
    chipbox.run(event_loop)
}
