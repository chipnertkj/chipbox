use std::collections::HashMap;

use crate::config;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum PreserveLayout {
    #[default]
    Disabled,
    Enabled {
        /// Only required during saving. `None` during save will result in a panic.
        layout_opt: Option<super::UiLayout>,
    },
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct UiLayoutConfig {
    pub preserve_layout: PreserveLayout,
    pub selected_layout_name_opt: Option<String>,
    layouts: HashMap<String, super::UiLayout>,
}

impl UiLayoutConfig {
    pub fn layout(&self, name: &str) -> Option<&super::UiLayout> {
        self.layouts.get(name)
    }

    pub fn layout_mut(&mut self, name: &str) -> Option<&mut super::UiLayout> {
        self.layouts.get_mut(name)
    }

    pub fn selected_layout_opt(&self) -> Option<&super::UiLayout> {
        let preserved_opt = match self.preserve_layout {
            PreserveLayout::Disabled => None,
            PreserveLayout::Enabled { ref layout_opt } => layout_opt.as_ref(),
        };

        // Cloning isn't ideal.
        let selected_opt = self
            .selected_layout_name_opt
            .clone()
            .and_then(|x| self.layouts.get(&x));

        preserved_opt.or(selected_opt)
    }

    pub fn selected_layout_opt_mut(&mut self) -> Option<&mut super::UiLayout> {
        let preserved_opt = match self.preserve_layout {
            PreserveLayout::Disabled => None,
            PreserveLayout::Enabled { ref mut layout_opt } => {
                layout_opt.as_mut()
            }
        };

        // Cloning isn't ideal.
        let selected_opt = self
            .selected_layout_name_opt
            .clone()
            .and_then(|x| self.layouts.get_mut(&x));

        preserved_opt.or(selected_opt)
    }

    pub fn selected_or_default(&self) -> super::UiLayout {
        match self.selected_layout_opt() {
            Some(layout) => layout.clone(),
            None => Default::default(),
        }
    }

    pub fn enable_preserve_layout(&mut self, value: bool) {
        if value {
            self.preserve_layout = PreserveLayout::Enabled { layout_opt: None }
        } else {
            self.preserve_layout = PreserveLayout::Disabled
        }
    }

    /// This is a no-op if `UiLayout::preserve_layout` is `Disabled`.
    pub fn set_preserved_layout_if_enabled(&mut self, layout: super::UiLayout) {
        if let PreserveLayout::Enabled { ref mut layout_opt } =
            self.preserve_layout
        {
            *layout_opt = Some(layout)
        }
    }

    pub fn insert_layout(
        &mut self,
        name: String,
        layout: super::UiLayout,
    ) -> Option<super::UiLayout> {
        self.layouts
            .insert(name, layout)
    }
}

impl config::TomlConfigTrait for UiLayoutConfig {
    fn config_file_name() -> &'static str {
        "ui_layout_config.toml"
    }

    /// Ensures `PreserveLayout::Yes::layout_opt` is `Some(_)` if `Self::preserve_layout` is set to `PreserveLayout::Yes`.
    /// # Panics
    /// Panics if `PreserveLayout::Yes::layout_opt` is `None` during save.
    fn validate_and_fix(
        &mut self,
        validation_type: config::ValidationType,
    ) -> bool {
        match validation_type {
            config::ValidationType::Save => {
                if let PreserveLayout::Enabled { ref layout_opt } =
                    self.preserve_layout
                {
                    if layout_opt.is_none() {
                        panic!("PreserveLayout::Yes::layout_opt was None");
                    }
                }
            }
            config::ValidationType::Load => {
                if let PreserveLayout::Enabled { ref layout_opt } =
                    self.preserve_layout
                {
                    if layout_opt.is_none() {
                        tracing::error!("PreserveLayout::Yes::layout_opt was None! Reverting to default.");
                        self.preserve_layout = PreserveLayout::Disabled;
                        return false;
                    }
                }
            }
        }
        true
    }
}
