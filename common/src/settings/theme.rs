use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Theme {}

impl Theme {
    const fn chipbox_dark() -> Self {
        Self {}
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq,
)]
pub enum ThemeSelector {
    #[default]
    ChipboxDark,
    Custom(String),
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default,
)]
pub struct UserThemes {
    inner: HashMap<String, Theme>,
}

impl UserThemes {
    pub fn theme(&self, selector: &ThemeSelector) -> Option<&Theme> {
        static CHIPBOX_DARK: Theme = Theme::chipbox_dark();
        match selector {
            ThemeSelector::ChipboxDark => Some(&CHIPBOX_DARK),
            ThemeSelector::Custom(custom) => self.inner.get(custom),
        }
    }
}
