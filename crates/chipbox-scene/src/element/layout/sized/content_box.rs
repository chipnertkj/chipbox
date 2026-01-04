use crate::{ElementId, ElementNode};

/// A content box element is a container for other elements.
/// It is a sized element &mdash;
/// it calculates its size based on its descendant's content size.
#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ContentBoxElement {
    #[serde(default)]
    pub id: Option<ElementId>,
    #[serde(default)]
    pub children: Vec<ElementNode>,
}

impl ContentBoxElement {
    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }
}
