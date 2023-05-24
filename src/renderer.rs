mod window;

pub use window::*;

pub struct Renderer {
    render_context: vello::util::RenderContext,
    pub windows: Windows,
}

impl Renderer {
    pub async fn new(
        primary_window: winit::window::Window,
    ) -> (Self, WindowId) {
        let mut render_context = vello::util::RenderContext::new()
            .expect("unable to construct vello render context");

        let surface_size = primary_window.inner_size();
        let mut render_surface = render_context
            .create_surface(
                &primary_window,
                surface_size.width,
                surface_size.height,
            )
            .await;
        render_context.set_present_mode(
            &mut render_surface,
            wgpu::PresentMode::AutoVsync,
        );

        let main_window = Window {
            inner: primary_window,
            render_surface,
        };
        let (windows, main_window_id) = Windows::new(main_window);

        let s = Self {
            render_context,
            windows,
        };

        (s, main_window_id)
    }
}
