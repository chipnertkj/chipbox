pub use theme::Theme;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
mod theme;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Settings {
    user_themes: HashMap<String, Theme>,
    recent_projects: Vec<PathBuf>,
}
