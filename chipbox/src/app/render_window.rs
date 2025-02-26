use chipnertkj_ui_render::{
    Renderer,
    renderer::{RendererError, SurfaceConfigError},
};
use std::sync::Arc;

/// Window handle with a [`Renderer`] attached to it.
pub(crate) struct RenderWindow<'window> {
    /// Label for logging.
    label: &'static str,
    /// Inner window handle.
    /// Ownership shared with the renderer.
    window: Arc<winit::window::Window>,
    /// Renderer instance used for displaying the contents.
    renderer: Renderer<'window>,
}

impl RenderWindow<'_> {
    /// Create a new window with the provided event loop and attributes.
    pub(crate) async fn with_ev_loop(
        label: &'static str,
        event_loop: &winit::event_loop::ActiveEventLoop,
        attributes: winit::window::WindowAttributes,
    ) -> miette::Result<Self> {
        let window = event_loop
            .create_window(attributes)
            .expect("failed to create window");
        let window = Arc::new(window);
        let renderer = Renderer::new(window.clone(), window.inner_size()).await?;
        Ok(Self {
            label,
            window,
            renderer,
        })
    }

    /// Get the identifier of the inner window.
    pub(crate) fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    /// Render a new frame and present it to the window.
    pub(crate) fn render_frame(&mut self) {
        match self.renderer.render_frame() {
            Ok(()) => (),
            Err(RendererError::SurfaceConfig(SurfaceConfigError::InvalidSize)) => {
                tracing::warn!("{}: surface size invalid", self.label);
            }
            Err(e) => {
                tracing::error!("{}: renderer error: {e:?}", self.label);
            }
        }
    }

    /// Update the size of the renderer.
    pub(crate) fn update_renderer_size(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.set_size(size);
    }
}
