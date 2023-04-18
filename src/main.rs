use std::{env, fs, path};
use tracing_subscriber::util::SubscriberInitExt as _;
use winit::{dpi, event, event_loop, window};

mod renderer;

#[derive(serde::Serialize, serde::Deserialize)]
struct WindowSettings {
    pub logical_size_unmaximized: dpi::LogicalSize<f32>,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            logical_size_unmaximized: dpi::LogicalSize::new(800., 600.),
        }
    }
}

impl<'de> Settings<'de> for WindowSettings {
    fn config_file_name() -> &'static str {
        "window_settings.toml"
    }
}

trait Settings<'de>: serde::Serialize + serde::Deserialize<'de> {
    fn config_file_name() -> &'static str;
    fn config_file_path() -> path::PathBuf {
        Self::config_folder().join(Self::config_file_name())
    }
    fn config_folder() -> path::PathBuf {
        home::home_dir()
            .expect("expected the HOME env var to be set")
            .join(".chipbox")
    }

    fn load() -> anyhow::Result<WindowSettings> {
        let toml = fs::read_to_string(Self::config_file_path())?;
        let settings = toml::from_str(toml.as_str())?;
        Ok(settings)
    }

    fn save(&self) -> anyhow::Result<()> {
        let config_folder = Self::config_folder();
        if !config_folder.exists() {
            fs::create_dir(config_folder)?;
        }
        let toml = toml::to_string(self)?;
        fs::write(Self::config_file_path(), toml)?;
        Ok(())
    }
}

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder().finish();
    subscriber.init();

    let title = {
        const TITLE_BASE: &str = "Chipbox";
        match option_env!("CARGO_PKG_VERSION") {
            Some(version) => format!("{TITLE_BASE} {version}"),
            None => TITLE_BASE.into(),
        }
    };
    let mut window_settings = match WindowSettings::load() {
        Ok(settings) => {
            tracing::info!("Successfully loaded window settings.");
            settings
        }
        Err(e) => {
            tracing::warn!(
                "Unable to load window settings, using default config: {e}"
            );
            Default::default()
        }
    };

    let event_loop = event_loop::EventLoop::default();
    let main_window = window::WindowBuilder::new()
        .with_inner_size(window_settings.logical_size_unmaximized)
        .with_title(title)
        .build(&event_loop)
        .expect("program should be able to create a window");

    let mut renderer = renderer::Renderer::new(&main_window);

    event_loop.run(move |event, _target, control_flow| match event {
        event::Event::WindowEvent { window_id, event }
            if window_id == main_window.id() =>
        {
            match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit
                }
                event::WindowEvent::Resized(physical_size) => {
                    renderer.resize_main_surface(&physical_size);
                    if !main_window.is_maximized() {
                        window_settings.logical_size_unmaximized =
                            physical_size.to_logical(main_window.scale_factor())
                    }
                }
                _ => {}
            }
        }
        event::Event::RedrawRequested(window_id)
            if window_id == main_window.id() =>
        {
            match renderer.render() {
                Ok(_) => { /* :) */ }
                Err(e) => match e {
                    wgpu::SurfaceError::OutOfMemory => {
                        tracing::error!("Out of memory.");
                        *control_flow = event_loop::ControlFlow::Exit;
                    }
                    wgpu::SurfaceError::Lost => renderer.reconfigure_surface(),
                    _ => tracing::warn!("Redraw error: '{e}'"),
                },
            }
        }
        event::Event::LoopDestroyed => match window_settings.save() {
            Ok(_) => tracing::info!("Successfully saved window settings."),
            Err(e) => tracing::error!("Unable to save window settings: {e}"),
        },
        _ => {}
    });
}
