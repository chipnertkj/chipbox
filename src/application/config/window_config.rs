use winit::dpi;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WindowConfig {
    pub logical_size_unmaximized: dpi::LogicalSize<f32>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            logical_size_unmaximized: dpi::LogicalSize::new(800., 600.),
        }
    }
}

impl super::ConfigTrait for WindowConfig {
    fn config_file_name() -> &'static str {
        "window_config.toml"
    }
}
