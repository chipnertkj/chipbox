use crate::config;
use winit::dpi;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ApplicationConfig {
    pub logical_size_unmaximized: dpi::LogicalSize<f32>,
}

impl ApplicationConfig {
    const DEFAULT_SIZE: dpi::LogicalSize<f32> =
        dpi::LogicalSize::new(800., 600.);
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            logical_size_unmaximized: Self::DEFAULT_SIZE,
        }
    }
}

impl config::TomlConfigTrait for ApplicationConfig {
    fn config_file_name() -> &'static str {
        "application_config.toml"
    }

    fn validate_and_fix(
        &mut self,
        _validation_type: config::ValidationType,
    ) -> bool {
        let size = self.logical_size_unmaximized;
        if size.width <= 0. || size.height <= 0. {
            tracing::error!("ApplicationConfig::logical_size_unmaximized is zero or negative. Using default window size.");
            self.logical_size_unmaximized = Self::DEFAULT_SIZE;
            return false;
        }
        true
    }
}
