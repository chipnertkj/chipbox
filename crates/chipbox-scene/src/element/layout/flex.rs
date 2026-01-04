mod item;

pub use self::item::FlexItemElement;
use crate::{Axis, ElementId, LinearDirection};

#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FlexElement {
    #[serde(default)]
    pub id: Option<ElementId>,
    /// May only contain flex item elements (for proportional distribution).
    #[serde(default)]
    pub children: Vec<FlexItemElement>,
    /// On which axis the elements are distributed.
    #[serde(default = "FlexElement::default_axis")]
    pub axis: Axis,
    /// The direction in which the elements are distributed along the axis.
    #[serde(default)]
    pub direction: LinearDirection,
}

impl FlexElement {
    #[must_use]
    pub const fn default_axis() -> Axis {
        Axis::Horizontal
    }

    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }
}
