use const_format::concatcp;
use miette::{Context as _, IntoDiagnostic};
use render_window::RenderWindow;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowAttributes;

mod render_window;

pub(crate) struct App<'app> {
    rt: tokio::runtime::Runtime,
    main_window: Option<RenderWindow<'app>>,
}

impl App<'_> {
    /// The title of the main window.
    const MAIN_WINDOW_TITLE: &'static str =
        concatcp!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

    /// Initialize the main window.
    fn init_main_window(&mut self, event_loop: &ActiveEventLoop) -> miette::Result<()> {
        let mut attributes = WindowAttributes::default();
        attributes.title = Self::MAIN_WINDOW_TITLE.to_string();
        let fut = RenderWindow::with_ev_loop(event_loop, attributes);
        let render_window = self.rt.block_on(fut)?;
        self.main_window = Some(render_window);
        Ok(())
    }

    /// Handle close request for the main window.
    fn main_close_request(&mut self, event_loop: &ActiveEventLoop) {
        self.main_window = None;
        event_loop.exit();
    }

    /// Initialize the application.
    pub(crate) fn new() -> Self {
        let rt = tokio::runtime::Runtime::new().expect("init tokio");
        Self {
            rt,
            main_window: None,
        }
    }

    /// Run the application.
    pub(crate) fn run(&mut self) -> miette::Result<()> {
        let event_loop = EventLoop::new()
            .into_diagnostic()
            .wrap_err("failed to create winit event loop")?;
        event_loop
            .run_app(self)
            .into_diagnostic()
            .wrap_err("failed to run winit app")?;
        Ok(())
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.init_main_window(event_loop).expect("init main window");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(ref mut render_window) = self.main_window {
            if render_window.id() == window_id {
                match event {
                    WindowEvent::CloseRequested => self.main_close_request(event_loop),
                    WindowEvent::RedrawRequested => {
                        render_window.redraw();
                    }
                    WindowEvent::Resized(physical_size) => {
                        render_window.update_renderer_size(physical_size);
                    }
                    _ => {}
                }
            }
        }
    }
}
