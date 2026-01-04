mod behavior;
mod content;
mod layout;

use std::sync::Arc;

use delegate_match::delegate_match;

pub use self::{
    behavior::{BehaviorElement, TextureElement},
    content::{Color, ContentElement, TextElement},
    layout::{
        ArrayElement, Axis, BoxElement, ContentBoxElement, FlexElement, FlexItemElement,
        GridElement, LayoutElement, LayoutLength, LinearDirection, MarginElement, SizedElement,
    },
};

#[derive(specta::Type, Clone)]
pub struct ElementId(pub Arc<str>);

#[derive(specta::Type)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum ElementNode {
    #[serde(flatten)]
    Layout(LayoutElement),
    #[serde(flatten)]
    Behavior(BehaviorElement),
    #[serde(flatten)]
    Content(ContentElement),
}

impl ElementNode {
    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        delegate_match! { match self {
            Self::{ Layout, Behavior, Content }(e) => e.id(),
        }}
    }
}
