pub use theme::{Theme, ThemeSelector, UserThemes};
pub mod audio_engine;

mod theme;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Settings {
    pub user_themes: UserThemes,
    pub selected_theme: ThemeSelector,
    pub recent_projects: Vec<PathBuf>,
    pub audio_engine: audio_engine::Settings,
}
