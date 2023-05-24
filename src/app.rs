use winit::{event_loop, window};

use crate::renderer;

pub struct App {
    renderer: renderer::Renderer,
    primary_window_id: renderer::WindowId,
    secondary_window_ids: Vec<renderer::WindowId>,
}

impl App {
    /// Calls `winit::event_loop::EventLoop::<T>::run`, taking over the current thread.
    /// # Notes
    /// Any value not moved to this function before calling it will not be dropped.
    pub async fn run<T>(self, event_loop: event_loop::EventLoop<T>) -> ! {
        event_loop.run(move |_event, _, control_flow| {
            *control_flow = event_loop::ControlFlow::Exit;
        })
    }

    pub async fn new<T>(
        window_target: &event_loop::EventLoopWindowTarget<T>,
    ) -> Self {
        let primary_window = window::WindowBuilder::default()
            .build(window_target)
            .expect("unable to create main window");

        let (renderer, primary_window_id) =
            renderer::Renderer::new(primary_window).await;
        let secondary_window_ids = vec![];

        Self {
            renderer,
            primary_window_id,
            secondary_window_ids,
        }
    }
}
