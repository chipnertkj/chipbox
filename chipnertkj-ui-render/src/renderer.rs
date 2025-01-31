use miette::{Context as _, IntoDiagnostic as _, miette};

pub struct Renderer<'target> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'target>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    cached_config: Option<wgpu::SurfaceConfiguration>,
    is_surface_configured: bool,
    size: nalgebra_glm::U32Vec2,
}

impl<'target> Renderer<'target> {
    pub async fn new(
        target: impl Into<wgpu::SurfaceTarget<'target>>,
        size: impl Into<mint::Vector2<u32>>,
    ) -> miette::Result<Self> {
        let size = size.into().into();
        let instance = Self::new_instance();
        let surface = Self::create_surface(&instance, target)?;
        let adapter = Self::request_adapter(&instance, &surface).await?;
        let (device, queue) = Self::request_device(&adapter).await?;
        let cached_config = Self::surface_configuration(&adapter, &surface, size)?;
        let mut is_surface_configured = false;
        if let Some(ref config) = cached_config {
            surface.configure(&device, config);
            is_surface_configured = true;
        };
        Ok(Self {
            size,
            instance,
            surface,
            adapter,
            device,
            queue,
            cached_config,
            is_surface_configured,
        })
    }

    /// Attempts to render a frame to the surface.
    ///
    /// Returns `Ok(true)` if the frame was rendered successfully, `Ok(false)` if rendering was skipped
    /// due to invalid configuration or size.
    /// Returns a [`wgpu::SurfaceError`] if encountered an error accessing the surface.
    pub fn render_pass(&self) -> Result<bool, wgpu::SurfaceError> {
        if !self.is_surface_configured {
            return Ok(false);
        }
        let surface_texture = self.surface.get_current_texture()?;
        let view = Self::surface_texture_view(&surface_texture);
        let mut encoder = self.render_encoder();
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                    a: 1.,
                }),
                store: wgpu::StoreOp::Store,
            },
        };
        let desc = wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(color_attachment)],
            ..Default::default()
        };
        encoder.begin_render_pass(&desc);
        let render_pass = encoder.begin_render_pass(&desc);
        drop(render_pass);
        let command_buffer = std::iter::once(encoder.finish());
        let _ix = self.queue.submit(command_buffer);
        surface_texture.present();
        Ok(true)
    }

    pub fn set_size(&mut self, size: impl Into<mint::Vector2<u32>>) {
        self.is_surface_configured = false;
        self.size = size.into().into();
        if let Some(ref mut config) = self.cached_config {
            config.width = self.size.x;
            config.height = self.size.y;
            if Self::is_surface_size_vaild(&self.size) {
                self.surface.configure(&self.device, config);
                self.is_surface_configured = true
            }
        };
    }

    fn is_surface_size_vaild(size: &nalgebra_glm::U32Vec2) -> bool {
        size.x != 0 && size.y != 0
    }

    fn render_encoder(&self) -> wgpu::CommandEncoder {
        let desc = &wgpu::CommandEncoderDescriptor {
            label: Some("render encoder"),
        };
        self.device.create_command_encoder(desc)
    }

    fn surface_texture_view(surface_texture: &wgpu::SurfaceTexture) -> wgpu::TextureView {
        let desc = Default::default();
        surface_texture.texture.create_view(&desc)
    }

    fn new_instance() -> wgpu::Instance {
        let desc = Default::default();
        wgpu::Instance::new(&desc)
    }

    fn create_surface(
        instance: &wgpu::Instance,
        target: impl Into<wgpu::SurfaceTarget<'target>>,
    ) -> miette::Result<wgpu::Surface<'target>> {
        instance
            .create_surface(target)
            .into_diagnostic()
            .wrap_err("failed to create wgpu surface")
    }

    async fn request_adapter(
        instance: &wgpu::Instance,
        compatible_surface: &wgpu::Surface<'target>,
    ) -> miette::Result<wgpu::Adapter> {
        let options = wgpu::RequestAdapterOptionsBase {
            compatible_surface: Some(compatible_surface),
            ..Default::default()
        };
        instance
            .request_adapter(&options)
            .await
            .ok_or(miette!("no valid wgpu adapter"))
    }

    async fn request_device(
        adapter: &wgpu::Adapter,
    ) -> miette::Result<(wgpu::Device, wgpu::Queue)> {
        let required_features = wgpu::Features::empty();
        let desc = wgpu::DeviceDescriptor {
            required_features,
            ..Default::default()
        };
        adapter
            .request_device(&desc, None)
            .await
            .into_diagnostic()
            .wrap_err("no valid wgpu device")
    }

    /// Returns `None` if either coordinate of `size` is `0`.
    fn surface_configuration(
        adapter: &wgpu::Adapter,
        surface: &wgpu::Surface,
        size: nalgebra_glm::U32Vec2,
    ) -> miette::Result<Option<wgpu::SurfaceConfiguration>> {
        // Size cannot be zero for a `SurfaceTexture`.
        if size.x == 0 || size.y == 0 {
            // Invalid size, return no config.
            return Ok(None);
        }
        // Size is valid, continue surface config.
        let caps = surface.get_capabilities(adapter);
        // Get default config for this surface.
        let mut config = surface
            .get_default_config(adapter, size.x, size.y)
            .ok_or(miette!("surface not compatible with adapter"))?;
        // Enable VSync.
        // `AutoVsync` is supported everywhere.
        config.present_mode = wgpu::PresentMode::AutoVsync;
        // Require sRGB (for now).
        // TODO: support other color spaces.
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .ok_or(miette!("srgb not supported"))?;
        config.format = format;
        Ok(Some(config))
    }
}
