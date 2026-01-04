use std::num::NonZeroUsize;

use crate::{Axis, BoxElement, ElementId, LinearDirection};

/// A grid element is a container for other elements.
/// It is a layout element that distributes its children in a grid.
#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct GridElement {
    #[serde(default)]
    pub id: Option<ElementId>,
    #[serde(default)]
    pub children: Vec<BoxElement>,
    /// The main axis along which the elements are distributed.
    #[serde(default = "GridElement::default_axis")]
    pub axis: Axis,
    /// The direction in which the elements are distributed along the axis.
    ///
    /// The layout of a grid of [`Self::array_limit`] = 1 will behave similarly to a [`FlexElement`].
    /// The direction of such a grid would be the same as the direction of the flex element.
    ///
    ///
    /// [`FlexElement`]: crate::element::layout::FlexElement
    #[serde(default)]
    pub direction: LinearDirection,
    /// The direction in which the elements are distributed perpendicular to the axis.
    ///
    /// When [`Self::axis`] is [`Axis::Vertical`], this is the distribution direction for rows,
    /// and vice versa for [`Axis::Horizontal`].
    #[serde(default)]
    pub array_direction: LinearDirection,
    /// How many items can fit perpendicular to the main axis.
    pub array_limit: NonZeroUsize,
}

impl GridElement {
    #[must_use]
    pub const fn default_axis() -> Axis {
        Axis::Vertical
    }

    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }
}
