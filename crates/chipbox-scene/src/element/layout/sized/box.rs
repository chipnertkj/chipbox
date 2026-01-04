use crate::{ElementId, ElementNode, LayoutLength};

/// A box element is a container for other elements.
/// It is a sized element &mdash; it defines its own width and height.
#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BoxElement {
    #[serde(default)]
    pub id: Option<ElementId>,
    #[serde(default)]
    pub children: Vec<ElementNode>,
    pub width: LayoutLength,
    pub height: LayoutLength,
}

impl BoxElement {
    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }
}
