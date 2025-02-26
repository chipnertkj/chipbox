#[cfg(feature = "hot")]
use crate::hot::ObserverHandle;
use const_format::concatcp;
use miette::{Context as _, IntoDiagnostic};
use render_window::RenderWindow;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowAttributes;

mod render_window;

/// Application state.
pub struct App<'app> {
    rt: tokio::runtime::Runtime,
    main_window: Option<RenderWindow<'app>>,
    #[cfg(feature = "hot")]
    observer: Option<ObserverHandle>,
}

impl App<'_> {
    /// Initialize the application.
    pub fn new() -> Self {
        let rt = tokio::runtime::Runtime::new().expect("init tokio");
        // Init hot-reload event observer.
        #[cfg(feature = "hot")]
        let tx = ObserverHandle::init(&rt, Self::on_hot_reload);
        Self {
            rt,
            main_window: None,
            // Add join handle to close thread on exit.
            #[cfg(feature = "hot")]
            observer: Some(tx),
        }
    }

    /// Run the application.
    pub fn run(&mut self) -> miette::Result<()> {
        let event_loop = EventLoop::new()
            .into_diagnostic()
            .wrap_err("failed to create winit event loop")?;
        event_loop
            .run_app(self)
            .into_diagnostic()
            .wrap_err("failed to run winit app")?;
        Ok(())
    }

    #[cfg(feature = "hot")]
    fn on_hot_reload() {
        tracing::info!("hot reload");
    }

    /// Initialize the main window.
    fn init_main_window(&mut self, event_loop: &ActiveEventLoop) -> miette::Result<()> {
        const MAIN_WINDOW_TITLE: &str =
            concatcp!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));
        let mut attributes = WindowAttributes::default();
        attributes.title = MAIN_WINDOW_TITLE.to_string();
        let fut = RenderWindow::with_ev_loop("main-window", event_loop, attributes);
        let render_window = self.rt.block_on(fut)?;
        self.main_window = Some(render_window);
        Ok(())
    }

    /// Handle close request for the main window.
    fn main_close_request(&mut self, event_loop: &ActiveEventLoop) {
        self.main_window = None;
        // Close hot-reload event observer thread.
        #[cfg(feature = "hot")]
        if let Some(tx) = self.observer.take() {
            tx.abort();
        }
        event_loop.exit();
    }
}

impl Default for App<'_> {
    fn default() -> Self {
        Self::new()
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
                        render_window.render_frame();
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
