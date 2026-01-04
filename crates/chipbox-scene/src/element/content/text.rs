use std::sync::Arc;

use crate::{Color, ElementId};

#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TextElement {
    #[serde(default)]
    pub id: Option<ElementId>,
    #[serde(default)]
    pub children: Vec<Arc<str>>,
    pub font: Arc<str>,
    pub weight: u16,
    /// Defines em square size.
    pub size: f32,
    pub color: Color,
}

impl TextElement {
    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }
}
