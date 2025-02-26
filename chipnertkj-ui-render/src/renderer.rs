use miette::{Context as _, IntoDiagnostic as _, miette};

#[derive(Debug, Clone, Copy, thiserror::Error)]
/// Error encountered when the surface is not supported.
pub enum SurfaceUnsupported {
    /// The surface is not compatible with the adapter.
    #[error("not compatible with adapter")]
    Incompatible,
    /// The surface does not support sRGB.
    #[error("does not support sRGB")]
    NoSrgb,
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
/// Error encountered during surface configuration.
pub enum SurfaceConfigError {
    /// The surface size is invalid.
    #[error("invalid size")]
    InvalidSize,
    /// The surface is not compatible with the adapter.
    #[error("unsupported")]
    Unsupported(#[from] SurfaceUnsupported),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum RendererError {
    /// The surface is misconfigured.
    #[error("surface misconfigured")]
    SurfaceConfig(#[from] SurfaceConfigError),
    /// Unable to access [`wgpu::Surface`].
    #[error("surface access error")]
    Surface(#[from] wgpu::SurfaceError),
}

/// WGPU-based renderer.
/// TODO: describe
pub struct Renderer<'target> {
    surface: wgpu::Surface<'target>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: Result<wgpu::SurfaceConfiguration, SurfaceConfigError>,
}

impl<'target> Renderer<'target> {
    /// Create a new renderer and configure it for the given surface and size.
    pub async fn new(
        target: impl Into<wgpu::SurfaceTarget<'target>>,
        size: impl Into<mint::Vector2<u32>>,
    ) -> miette::Result<Self> {
        let size = size.into();
        let instance = Self::instance();
        let surface = Self::create_surface(&instance, target)?;
        let adapter = Self::request_adapter(&instance, &surface).await?;
        let (device, queue) = Self::request_device(&adapter).await?;
        let surface_config = Self::configure_surface(&device, &adapter, &surface, &size);
        Ok(Self {
            surface,
            adapter,
            device,
            queue,
            surface_config,
        })
    }

    /// Attempts to render and present a frame to the surface.
    pub fn render_frame(&mut self) -> Result<(), RendererError> {
        if let Err(e) = self.surface_config {
            return Err(e.into());
        }
        let (surface_texture, view) = self.prepare_render_surface()?;
        self.render_pass(&view);
        surface_texture.present();
        Ok(())
    }

    /// Submits a render pass on the provided texture view.
    pub fn render_pass(&self, view: &wgpu::TextureView) {
        // Create a new command encoder for render commands.
        let mut encoder = self.create_render_encoder();
        // Prepare render pass descriptor.
        let color_attachment = wgpu::RenderPassColorAttachment {
            view,
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
            label: Some("ui-render-pass"),
            color_attachments: &[Some(color_attachment)],
            ..Default::default()
        };
        // Begin the render pass.
        let render_pass = encoder.begin_render_pass(&desc);
        // Finish the render pass.
        drop(render_pass);
        // Submit render pass to command queue.
        let command_buffer = std::iter::once(encoder.finish());
        let _ix = self.queue.submit(command_buffer);
    }

    /// Update the size of the surface and re-configure it if necessary.
    pub fn set_size(&mut self, size: impl Into<mint::Vector2<u32>>) {
        let size = size.into();
        // Update stored configuration.
        if Self::is_surface_size_vaild(&size) {
            // Reconfigure surface with new size.
            if let Ok(ref mut config) = self.surface_config {
                // Update and apply cached config.
                config.width = size.x;
                config.height = size.y;
                self.surface.configure(&self.device, config);
                tracing::info!("surface reconfigured with cached config");
            } else {
                // Surface is not configured, create a new config.
                tracing::debug!("surface not configured, creating new config");
                self.reconfigure_surface(&size);
            }
        } else {
            // Invalid size, set error state.
            self.surface_config = Err(SurfaceConfigError::InvalidSize);
        }
    }

    /// Checks if the provided size is valid for the surface.
    /// Both coordinates must be non-zero.
    fn is_surface_size_vaild(size: &mint::Vector2<u32>) -> bool {
        size.x != 0 && size.y != 0
    }

    /// Creates a new [`CommandEncoder`](wgpu::CommandEncoder) for rendering.
    fn create_render_encoder(&self) -> wgpu::CommandEncoder {
        let desc = &wgpu::CommandEncoderDescriptor {
            label: Some("ui-render-encoder"),
        };
        self.device.create_command_encoder(desc)
    }

    /// Prepare a surface for rendering.
    /// Returns the current [`SurfaceTexture`](wgpu::SurfaceTexture) to render to and its [`TextureView`](wgpu::TextureView).
    fn prepare_render_surface(
        &self,
    ) -> Result<(wgpu::SurfaceTexture, wgpu::TextureView), wgpu::SurfaceError> {
        // Get current surface texture.
        let surface_texture = self.surface.get_current_texture()?;
        // Create a new texture view for the surface texture.
        let desc = wgpu::TextureViewDescriptor {
            label: Some("ui-texture-view"),
            ..Default::default()
        };
        let view = surface_texture.texture.create_view(&desc);
        Ok((surface_texture, view))
    }

    /// Request an instance from [`wgpu`].
    fn instance() -> wgpu::Instance {
        let desc = Default::default();
        wgpu::Instance::new(&desc)
    }

    /// Creates a new unconfigured [`Surface`](wgpu::Surface) for the provided [`SurfaceTarget`](wgpu::SurfaceTarget) (e.g. compatible window).
    fn create_surface(
        instance: &wgpu::Instance,
        target: impl Into<wgpu::SurfaceTarget<'target>>,
    ) -> miette::Result<wgpu::Surface<'target>> {
        instance
            .create_surface(target)
            .into_diagnostic()
            .wrap_err("failed to create wgpu surface")
    }

    /// Requests an [`Adapter`](wgpu::Adapter) from the [`Instance`](wgpu::Instance).
    ///
    /// Selects an adapter that is compatible with the provided surface.
    async fn request_adapter(
        instance: &wgpu::Instance,
        compatible_surface: &wgpu::Surface<'target>,
    ) -> miette::Result<wgpu::Adapter> {
        // Prepare adapter options.
        let options = wgpu::RequestAdapterOptions {
            compatible_surface: Some(compatible_surface),
            ..Default::default()
        };
        // Request adapter from instance.
        instance
            .request_adapter(&options)
            .await
            .ok_or(miette!("no valid wgpu adapter"))
    }

    /// Requests a device and command queue from the adapter.
    async fn request_device(
        adapter: &wgpu::Adapter,
    ) -> miette::Result<(wgpu::Device, wgpu::Queue)> {
        // Prepare device descriptor.
        let required_features = wgpu::Features::empty();
        let desc = wgpu::DeviceDescriptor {
            required_features,
            ..Default::default()
        };
        // Request device from adapter.
        adapter
            .request_device(&desc, None)
            .await
            .into_diagnostic()
            .wrap_err("no valid wgpu device")
    }

    /// Reconfigures the surface with the provided size.
    /// See [`Self::configure_surface`] for more details.
    fn reconfigure_surface(&mut self, size: &mint::Vector2<u32>) {
        let config = Self::configure_surface(&self.device, &self.adapter, &self.surface, size);
        self.surface_config = config;
    }

    /// Queries surface capabilities and generates a new configuration,
    /// which is applied to the surface.
    ///
    /// Returns `None` if either coordinate of `size` is `0`.
    /// That means the surface cannot be configured for the provided size.
    fn configure_surface(
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
        surface: &wgpu::Surface,
        size: &mint::Vector2<u32>,
    ) -> Result<wgpu::SurfaceConfiguration, SurfaceConfigError> {
        tracing::debug!("querying surface configuration");
        // Size cannot be zero for a `SurfaceTexture`.
        if size.x == 0 || size.y == 0 {
            // Invalid size, return no config.
            return Err(SurfaceConfigError::InvalidSize);
        }
        // Size is valid, continue surface config.
        let caps = surface.get_capabilities(adapter);
        // Get default config for this surface.
        let mut config = surface
            .get_default_config(adapter, size.x, size.y)
            .ok_or(SurfaceUnsupported::Incompatible)?;
        // Enable VSync.
        // `AutoVsync` is supported everywhere.
        config.present_mode = wgpu::PresentMode::AutoVsync;
        // Select a supported format.
        let format = caps
            .formats
            .iter()
            .copied()
            // Require sRGB (for now).
            // TODO: support other color spaces.
            .find(|f| f.is_srgb())
            .ok_or(SurfaceUnsupported::NoSrgb)?;
        config.format = format;
        // Apply the new config to the surface.
        surface.configure(device, &config);
        tracing::info!("surface configured with queried config");
        Ok(config)
    }
}
