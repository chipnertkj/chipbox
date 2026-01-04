mod text;

use delegate_match::delegate_match;

pub use self::text::TextElement;
use crate::ElementId;

#[derive(specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Color {
    Rgb { r: u8, g: u8, b: u8 },
    Hsv { h: f32, s: f32, v: f32 },
    Hsl { h: f32, s: f32, l: f32 },
}

#[derive(specta::Type)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ContentElement {
    Text(TextElement),
}

impl ContentElement {
    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        delegate_match! { match self {
            Self::{ Text }(e) => e.id(),
        }}
    }
}
