use crate::{ElementId, ElementNode, LayoutLength};

/// An align element is a container for other elements.
/// It is a layout element that aligns its children inside its bounds.
#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AlignElement {
    #[serde(default)]
    pub id: Option<ElementId>,
    #[serde(default)]
    pub children: Vec<ElementNode>,
    #[serde(default = "AlignElement::default_align")]
    pub origin: Alignment,
    #[serde(default = "AlignElement::default_align")]
    pub target: Alignment,
    #[serde(default)]
    pub x_offset: Option<LayoutLength>,
    #[serde(default)]
    pub y_offset: Option<LayoutLength>,
}

macro_rules! align {
    ($vertical:tt, $horizontal:tt) => {
        Alignment {
            horizontal: align!($horizontal),
            vertical: align!($vertical),
        }
    };
    (left) => {
        HorizontalAlign::Left
    };
    (center) => {
        HorizontalAlign::Center
    };
    (right) => {
        HorizontalAlign::Right
    };
    (top) => {
        VerticalAlign::Top
    };
    (middle) => {
        VerticalAlign::Middle
    };
    (bottom) => {
        VerticalAlign::Bottom
    };
}

impl AlignElement {
    #[must_use]
    pub const fn default_align() -> Alignment {
        align!(middle, center)
    }

    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }
}

#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Alignment {
    pub horizontal: HorizontalAlign,
    pub vertical: VerticalAlign,
}

#[derive(specta::Type)]
#[serde(rename_all = "kebab-case")]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(specta::Type)]
#[serde(rename_all = "kebab-case")]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}
