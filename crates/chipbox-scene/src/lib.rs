mod element;

pub use self::element::{
    ArrayElement, Axis, BehaviorElement, BoxElement, Color, ContentBoxElement, ContentElement,
    ElementId, ElementNode, FlexElement, FlexItemElement, GridElement, LayoutElement, LayoutLength,
    LinearDirection, MarginElement, SizedElement, TextElement, TextureElement,
};

#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    #[serde(default)]
    pub children: Vec<ElementNode>,
    pub px_width: u32,
    pub px_height: u32,
    #[serde(default = "Scene::default_scale")]
    pub scale: f32,
}

impl Scene {
    #[must_use]
    pub const fn default_scale() -> f32 {
        1.0
    }
}
