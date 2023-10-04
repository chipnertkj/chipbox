pub use theme::{Theme, ThemeSelector, UserThemes};

mod theme;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Settings {
    pub user_themes: UserThemes,
    pub selected_theme: ThemeSelector,
    pub recent_projects: Vec<PathBuf>,
}
