use crate::{ElementId, ElementNode, LayoutLength};

/// A margin element is a container for other elements.
/// It is a sized element &mdash; it defines its size in relation to its parent.
#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MarginElement {
    #[serde(default)]
    pub id: Option<ElementId>,
    #[serde(default)]
    pub children: Vec<ElementNode>,
    /// Base margin value, may be overridden by consecutive margin properties.
    pub base: Option<LayoutLength>,
    /// The horizontal margin &mdash; overrides [`Self::base`].
    pub horizontal: Option<LayoutLength>,
    /// The left margin &mdash; overrides [`Self::base`].
    pub left: Option<LayoutLength>,
    /// The right margin &mdash; overrides [`Self::base`] and [`Self::horizontal`].
    pub right: Option<LayoutLength>,
    /// The vertical margin &mdash; overrides [`Self::base`] and [`Self::horizontal`].
    pub vertical: Option<LayoutLength>,
    /// The top margin &mdash; overrides [`Self::base`] and [`Self::vertical`].
    pub top: Option<LayoutLength>,
    /// The bottom margin &mdash; overrides [`Self::base`] and [`Self::vertical`].
    pub bottom: Option<LayoutLength>,
}

impl MarginElement {
    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }
}
