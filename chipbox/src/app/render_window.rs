use chipnertkj_ui_render::Renderer;
use std::sync::Arc;

pub(crate) struct RenderWindow<'window> {
    inner: Arc<winit::window::Window>,
    renderer: Renderer<'window>,
}

impl RenderWindow<'_> {
    pub(crate) async fn with_ev_loop(
        event_loop: &winit::event_loop::ActiveEventLoop,
        attributes: winit::window::WindowAttributes,
    ) -> miette::Result<Self> {
        let window = event_loop
            .create_window(attributes)
            .expect("failed to create window");
        let window = Arc::new(window);
        let renderer = Renderer::new(window.clone(), window.inner_size()).await?;
        Ok(Self {
            inner: window,
            renderer,
        })
    }

    pub(crate) fn id(&self) -> winit::window::WindowId {
        self.inner.id()
    }

    pub(crate) fn redraw(&mut self) {
        self.inner.request_redraw();
        if self.renderer.is_configured() {
            let result = self.renderer.render_pass();
            result.expect("surface error")
        }
    }

    pub(crate) fn update_renderer_size(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.set_size(size)
    }
}
