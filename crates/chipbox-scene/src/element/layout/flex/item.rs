use crate::element::{ElementId, ElementNode};

#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FlexItemElement {
    #[serde(default)]
    pub id: Option<ElementId>,
    #[serde(default)]
    pub children: Vec<ElementNode>,
    /// The proportion of the flex item in relation to the rest.
    #[serde(default = "FlexItemElement::default_proportion")]
    pub proportion: f32,
}

impl FlexItemElement {
    #[must_use]
    pub const fn default_proportion() -> f32 {
        1.0
    }

    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }
}
