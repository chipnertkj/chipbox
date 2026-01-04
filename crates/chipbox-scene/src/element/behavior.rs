mod texture;

use delegate_match::delegate_match;

pub use self::texture::TextureElement;
use crate::ElementId;

/// Applies behavior to the parent element.
#[derive(specta::Type)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum BehaviorElement {
    Texture(TextureElement),
    SelectionContainer,
    SelectionAction,
    Selection,
    Deselection,
    PointerAction,
    PointerHoverMove,
    PointerEnter,
    PointerLeave,
}

impl BehaviorElement {
    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        delegate_match! { match self {
            Self::{ Texture }(e) => e.id(),
            Self::{
                SelectionContainer, SelectionAction, Selection, Deselection,
                PointerAction, PointerHoverMove, PointerEnter, PointerLeave
            } => None,
        }}
    }
}
