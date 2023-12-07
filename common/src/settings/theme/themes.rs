mod chipbox_dark;

use super::Theme;
use once_cell::sync::Lazy;

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq,
)]
pub enum DefaultThemeSelector {
    #[default]
    ChipboxDark,
}

pub fn get(selector: &DefaultThemeSelector) -> &'static Theme {
    static CHIPBOX_DARK: Lazy<Theme> = Lazy::new(chipbox_dark::theme);
    match selector {
        DefaultThemeSelector::ChipboxDark => &CHIPBOX_DARK,
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ThemeSelector {
    Default(DefaultThemeSelector),
    Custom(String),
}

impl Default for ThemeSelector {
    fn default() -> Self {
        ThemeSelector::Default(DefaultThemeSelector::default())
    }
}
