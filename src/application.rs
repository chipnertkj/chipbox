use winit::{dpi, event, event_loop, window};

mod application_config;
mod audio_engine;
mod config;
mod renderer;

use config::ConfigTrait as _;

pub struct Chipbox {
    window: window::Window,
    application_config: application_config::ApplicationConfig,
    renderer: renderer::Renderer,
    audio_engine: audio_engine::AudioEngine,
}

impl Chipbox {
    pub fn load_from_config<T>(event_loop: &event_loop::EventLoop<T>) -> Self {
        let application_config =
            application_config::ApplicationConfig::load_or_default_tracing();
        let window = window::WindowBuilder::new()
            .with_inner_size(application_config.logical_size_unmaximized)
            .with_title(Self::construct_title())
            .build(event_loop)
            .expect("program should be able to create a window");

        let renderer = renderer::Renderer::new(&window);
        let audio_engine = audio_engine::AudioEngine::load_from_config();

        Self {
            window,
            application_config,
            renderer,
            audio_engine,
        }
    }

    pub fn run<T>(mut self, event_loop: event_loop::EventLoop<T>) -> ! {
        event_loop.run(move |event, _target, control_flow| match event {
            event::Event::WindowEvent { window_id, event }
                if window_id == self.window.id() =>
            {
                match event {
                    event::WindowEvent::CloseRequested => {
                        self.on_close_requested(control_flow)
                    }
                    event::WindowEvent::Resized(physical_size) => {
                        self.on_resized(physical_size)
                    }
                    _ => {}
                }
            }
            event::Event::RedrawRequested(window_id)
                if window_id == self.window.id() =>
            {
                self.on_redraw_requested(control_flow)
            }
            event::Event::LoopDestroyed => self.on_exit(),
            _ => {}
        });
    }

    fn on_exit(&mut self) {
        self.application_config
            .save_tracing()
    }

    fn on_redraw_requested(
        &mut self,
        control_flow: &mut event_loop::ControlFlow,
    ) {
        match self.renderer.render() {
            Ok(_) => { /* :) */ }
            Err(e) => match e {
                wgpu::SurfaceError::OutOfMemory => {
                    tracing::error!("Out of memory.");
                    *control_flow = event_loop::ControlFlow::Exit;
                }
                wgpu::SurfaceError::Lost => self
                    .renderer
                    .reconfigure_main_surface(),
                _ => tracing::warn!("Redraw error: '{e}'"),
            },
        }
    }

    fn on_resized(&mut self, physical_size: dpi::PhysicalSize<u32>) {
        self.renderer
            .resize_main_surface(&physical_size);
        if !self.window.is_maximized() {
            self.application_config
                .logical_size_unmaximized =
                physical_size.to_logical(self.window.scale_factor())
        }
    }

    fn on_close_requested(&self, control_flow: &mut event_loop::ControlFlow) {
        *control_flow = event_loop::ControlFlow::Exit
    }

    fn construct_title() -> String {
        const TITLE_BASE: &str = "Chipbox";
        match option_env!("CARGO_PKG_VERSION") {
            Some(version) => format!("{TITLE_BASE} {version}"),
            None => TITLE_BASE.into(),
        }
    }
}
