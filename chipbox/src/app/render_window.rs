use chipnertkj_ui_render::Renderer;
use std::sync::Arc;

pub(crate) struct RenderWindow<'window> {
    inner: Arc<winit::window::Window>,
    label: &'static str,
    renderer: Renderer<'window>,
}

impl RenderWindow<'_> {
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
            inner: window,
            renderer,
        })
    }

    pub(crate) fn id(&self) -> winit::window::WindowId {
        self.inner.id()
    }

    pub(crate) fn redraw(&mut self) {
        let rendered = self.renderer.render_pass().unwrap_or_else(|e| {
            tracing::error!("{}: renderer error: {e:?}", self.label);
            false
        });
        if !rendered {
            tracing::warn!("{}: skipped frame", self.label);
        }
    }

    pub(crate) fn update_renderer_size(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.set_size(size)
    }
}
