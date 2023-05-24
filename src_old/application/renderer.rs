// use async_std::task;
// use itertools::Itertools as _;
// use winit::window;

// mod index;
// mod vertex;

// pub use index::Index;
// pub use vertex::Vertex;

// #[allow(unused)]
// pub struct Renderer {
//     instance: wgpu::Instance,
//     main_surface: wgpu::Surface,
//     device: wgpu::Device,
//     queue: wgpu::Queue,
//     main_surface_config: wgpu::SurfaceConfiguration,
//     default_shader: wgpu::ShaderModule,
//     render_pipeline: wgpu::RenderPipeline,
//     vertex_buffer_opt: Option<wgpu::Buffer>,
//     index_buffer_opt: Option<wgpu::Buffer>,
//     vertex_buffer_length: u32,
//     index_buffer_length: u32,
// }

// impl Renderer {
//     pub fn new(windows: &[window::Window]) -> Self {
//         let instance = Self::create_instance();

//         // # Safety:
//         //
//         let window_surfaces = windows
//             .iter()
//             .map(|x| unsafe { Self::create_window_surface(&instance, x) })
//             .collect_vec();

//         let adapters = window_surfaces
//             .iter()
//             .map(|x| Self::create_adapter(&instance, x))
//             .collect_vec();

//         let (devices, queues) = adapters
//             .iter()
//             .map(|x| Self::create_device(x))
//             .collect_vec()
//             .into_iter()
//             .unzip();

//         // let main_surface_config = Self::create_surface_config(
//         // window.inner_size(),
//         // &adapter,
//         // &main_surface,
//         // );
//         // main_surface.configure(&device, &main_surface_config);
//         //
//         // let default_shader = device
//         // .create_shader_module(wgpu::include_wgsl!("shaders/default.wgsl"));
//         //
//         // let render_pipeline = Self::create_render_pipeline(
//         // &device,
//         // &main_surface_config,
//         // &default_shader,
//         // );
//         //
//         // let vertex_buffer_opt = None;
//         // let index_buffer_opt = None;
//         // let vertex_buffer_length = 0;
//         // let index_buffer_length = 0;
//         //
//         // Self {
//         // instance,
//         // main_surface,
//         // device,
//         // queue,
//         // main_surface_config,
//         // default_shader,
//         // render_pipeline,
//         // vertex_buffer_opt,
//         // index_buffer_opt,
//         // vertex_buffer_length,
//         // index_buffer_length,
//         // }
//     }

//     pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
//         let main_surface_texture = self
//             .main_surface
//             .get_current_texture()?;
//         let view = main_surface_texture
//             .texture
//             .create_view(&Default::default());
//         let mut encoder = self
//             .device
//             .create_command_encoder(&Default::default());

//         let color_attachment = wgpu::RenderPassColorAttachment {
//             view: &view,
//             ops: wgpu::Operations {
//                 load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
//                 store: true,
//             },
//             resolve_target: None,
//         };
//         let render_pass_desc = wgpu::RenderPassDescriptor {
//             color_attachments: &[Some(color_attachment)],
//             depth_stencil_attachment: None,
//             label: None,
//         };
//         let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

//         render_pass.set_pipeline(&self.render_pipeline);

//         // Draw if vertex buffer is set.
//         match &self.vertex_buffer_opt {
//             Some(vertex_buffer) => {
//                 render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
//                 // Draw with or without index buffer.
//                 match &self.index_buffer_opt {
//                     Some(index_buffer) => {
//                         render_pass.set_index_buffer(
//                             index_buffer.slice(..),
//                             Index::describe(),
//                         );
//                         render_pass.draw_indexed(
//                             0..self.index_buffer_length,
//                             0,
//                             0..1,
//                         );
//                     }
//                     None => {
//                         render_pass.draw(0..self.vertex_buffer_length, 0..1);
//                     }
//                 }
//             }
//             None => {}
//         }

//         // Drop `render_pass` to remove mutable borrow on `encoder`.
//         drop(render_pass);

//         self.queue
//             .submit(std::iter::once(encoder.finish()));
//         main_surface_texture.present();

//         Ok(())
//     }

//     pub fn resize_main_surface(
//         &mut self,
//         surface_size: &winit::dpi::PhysicalSize<u32>,
//     ) {
//         if surface_size.width != 0 && surface_size.height != 0 {
//             self.main_surface_config.width = surface_size.width;
//             self.main_surface_config
//                 .height = surface_size.height;
//             self.main_surface
//                 .configure(&self.device, &self.main_surface_config);
//         }
//     }

//     pub fn reconfigure_main_surface(&self) {
//         self.main_surface
//             .configure(&self.device, &self.main_surface_config);
//     }

//     fn create_instance() -> wgpu::Instance {
//         let instance_desc = Default::default();
//         wgpu::Instance::new(instance_desc)
//     }

//     /// # Safety:
//     /// `window` has to live at least as long as the surface.
//     unsafe fn create_window_surface(
//         instance: &wgpu::Instance,
//         window: &window::Window,
//     ) -> wgpu::Surface {
//         unsafe {
//             instance
//                 .create_surface(window)
//                 .expect("instance was unable to create the main surface")
//         }
//     }

//     fn create_adapter(
//         instance: &wgpu::Instance,
//         surface: &wgpu::Surface,
//     ) -> wgpu::Adapter {
//         let adapter_options = wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::HighPerformance,
//             compatible_surface: Some(surface),
//             force_fallback_adapter: false,
//         };
//         task::block_on(instance.request_adapter(&adapter_options))
//             .expect("expected to find a suitable wgpu adapter")
//     }

//     fn create_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
//         let device_desc = wgpu::DeviceDescriptor {
//             features: wgpu::Features::empty(),
//             limits: wgpu::Limits::default(),
//             label: Some("chipbox_renderer_device_main"),
//         };
//         task::block_on(adapter.request_device(&device_desc, None))
//             .expect("unable to find a suitable wgpu device")
//     }

//     fn create_surface_config(
//         surface_size: winit::dpi::PhysicalSize<u32>,
//         adapter: &wgpu::Adapter,
//         surface: &wgpu::Surface,
//     ) -> wgpu::SurfaceConfiguration {
//         let surface_caps = surface.get_capabilities(adapter);
//         let surface_format = surface_caps
//             .formats
//             .iter()
//             .copied()
//             .find(|x| x.describe().srgb)
//             .unwrap_or(
//                 surface_caps
//                     .formats
//                     .first()
//                     .copied()
//                     .expect(
//                         "expected to find at least one compatible surface format",
//                     ),
//             );
//         if !surface_format.describe().srgb {
//             tracing::warn!("main surface is not srgb")
//         }
//         wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: surface_format,
//             width: surface_size.width,
//             height: surface_size.height,
//             present_mode: wgpu::PresentMode::AutoVsync,
//             alpha_mode: Default::default(),
//             view_formats: vec![],
//         }
//     }

//     fn create_render_pipeline(
//         device: &wgpu::Device,
//         surface_config: &wgpu::SurfaceConfiguration,
//         shader: &wgpu::ShaderModule,
//     ) -> wgpu::RenderPipeline {
//         let pipeline_layout =
//             device.create_pipeline_layout(&Default::default());
//         let fragment_targets = [Some(wgpu::ColorTargetState {
//             format: surface_config.format,
//             blend: Some(wgpu::BlendState::ALPHA_BLENDING),
//             write_mask: wgpu::ColorWrites::ALL,
//         })];
//         let pipeline_desc = wgpu::RenderPipelineDescriptor {
//             label: None,
//             layout: Some(&pipeline_layout),
//             vertex: wgpu::VertexState {
//                 module: shader,
//                 entry_point: "vs_main",
//                 buffers: &[Vertex::describe()],
//             },
//             fragment: Some(wgpu::FragmentState {
//                 module: shader,
//                 entry_point: "fs_main",
//                 targets: &fragment_targets,
//             }),
//             primitive: wgpu::PrimitiveState {
//                 topology: wgpu::PrimitiveTopology::TriangleList,
//                 strip_index_format: None,
//                 front_face: wgpu::FrontFace::Ccw,
//                 cull_mode: Some(wgpu::Face::Back),
//                 polygon_mode: wgpu::PolygonMode::Fill,
//                 unclipped_depth: false,
//                 conservative: false,
//             },
//             depth_stencil: None,
//             multisample: wgpu::MultisampleState {
//                 count: 1,
//                 mask: !0,
//                 alpha_to_coverage_enabled: false,
//             },
//             multiview: None,
//         };
//         device.create_render_pipeline(&pipeline_desc)
//     }
// }
