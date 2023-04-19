use winit::dpi;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WindowSettings {
    pub logical_size_unmaximized: dpi::LogicalSize<f32>,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            logical_size_unmaximized: dpi::LogicalSize::new(800., 600.),
        }
    }
}

impl super::SettingsTrait for WindowSettings {
    fn config_file_name() -> &'static str {
        "window_settings.toml"
    }
}
