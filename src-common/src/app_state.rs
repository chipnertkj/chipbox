mod project;
mod settings;

pub use project::*;
pub use settings::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum AppState {
    Setup {
        settings_result: Result<Settings, SettingsLoadError>,
    },
    Home {
        settings: Settings,
    },
    Editor {
        settings: Settings,
        project: Project,
    },
}
