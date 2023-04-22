use crate::config;
use winit::dpi;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub logical_size_unmaximized: dpi::LogicalSize<f32>,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            logical_size_unmaximized: dpi::LogicalSize::new(800., 600.),
        }
    }
}

impl config::ConfigTrait for ApplicationConfig {
    fn config_file_name() -> &'static str {
        "application_config.toml"
    }
}
