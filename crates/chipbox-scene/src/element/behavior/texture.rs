use crate::ElementId;

/// A texture element is a behavior element that makes its parent render to a texture.
#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TextureElement {
    #[serde(default)]
    pub id: Option<ElementId>,
    #[serde(default = "TextureElement::default_opacity")]
    pub opacity: f32,
    pub shader: (),
}

impl TextureElement {
    #[must_use]
    pub const fn default_opacity() -> f32 {
        1.0
    }

    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }
}
