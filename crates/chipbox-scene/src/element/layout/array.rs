use crate::{Axis, BoxElement, ElementId, LinearDirection};

#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ArrayElement {
    #[serde(default)]
    pub id: Option<ElementId>,
    /// May only contain box elements.
    #[serde(default)]
    pub children: Vec<BoxElement>,
    /// On which axis the elements are distributed.
    #[serde(default = "ArrayElement::default_axis")]
    pub axis: Axis,
    /// The direction in which the elements are distributed.
    #[serde(default)]
    pub direction: LinearDirection,
}

impl ArrayElement {
    #[must_use]
    pub const fn default_axis() -> Axis {
        Axis::Horizontal
    }

    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }
}
