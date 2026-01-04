mod align;
mod array;
mod flex;
mod grid;
mod sized;

use delegate_match::delegate_match;

pub use self::{
    align::AlignElement,
    array::ArrayElement,
    flex::{FlexElement, FlexItemElement},
    grid::GridElement,
    sized::{BoxElement, ContentBoxElement, MarginElement},
};
use crate::ElementId;

#[derive(specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LayoutLength {
    #[serde(rename = "su")]
    SceneUnit(f32),
    #[serde(rename = "sw")]
    SceneWidth(f32),
    #[serde(rename = "sh")]
    SceneHeight(f32),
    #[serde(rename = "pw")]
    ParentWidth(f32),
    #[serde(rename = "ph")]
    ParentHeight(f32),
    #[serde(rename = "px")]
    Pixel(f32),
    #[serde(rename = "mm")]
    Millimeter(f32),
    #[serde(rename = "cm")]
    Centimeter(f32),
    #[serde(rename = "in")]
    Inch(f32),
    #[serde(rename = "pt")]
    Point(f32),
}

#[specta::specta]
pub fn length_from_str(input: &str) -> Result<LayoutLength, LayoutUnitParseError<&str>> {
    LayoutLength::parse_from(input)
}

impl LayoutLength {
    /// Parse a layout unit from a string.
    /// ## Errors
    /// - [`EndOfInput`] if the input is empty
    /// - [`RecognizeFloat`] if the input is not a valid float.
    /// - [`Incomplete`] if the input is incomplete.
    /// ## Panics
    /// - If the input is a valid float but could not be parsed.
    ///   This would be indicative of a bug in this implementation.
    ///
    /// [`EndOfInput`]: LayoutUnitParseError::EndOfInput
    /// [`RecognizeFloat`]: LayoutUnitParseError::RecognizeFloat
    /// [`Incomplete`]: LayoutUnitParseError::Incomplete
    pub fn parse_from(input: &str) -> Result<Self, LayoutUnitParseError<&str>> {
        // Parse initial float.
        let (rest, value) = match nom::number::complete::recognize_float(input) {
            Ok((rest, value)) => {
                let float = value.parse().expect("recognized a valid float");
                Ok((rest, float))
            }
            Err(nom::Err::Error((i, _)) | nom::Err::Failure((i, _))) => {
                Err(LayoutUnitParseError::RecognizeFloat(i))
            }
            Err(nom::Err::Incomplete(_)) => Err(LayoutUnitParseError::Incomplete),
        }?;
        // Parse unit. If empty, assume scene units.
        let unit = match rest {
            "" | "su" => Self::SceneUnit(value),
            "sw" => Self::SceneWidth(value),
            "sh" => Self::SceneHeight(value),
            "pw" => Self::ParentWidth(value),
            "ph" => Self::ParentHeight(value),
            "px" => Self::Pixel(value),
            "mm" => Self::Millimeter(value),
            "cm" => Self::Centimeter(value),
            "in" => Self::Inch(value),
            "pt" => Self::Point(value),
            _ => return Err(LayoutUnitParseError::InvalidUnit(rest)),
        };
        Ok(unit)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LayoutUnitParseError<I> {
    #[error("end of input")]
    EndOfInput,
    #[error("invalid unit")]
    InvalidUnit(I),
    #[error("unable to recognize float")]
    RecognizeFloat(I),
    #[error("input is incomplete")]
    Incomplete,
}

#[derive(specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Axis {
    #[serde(alias = "x")]
    Horizontal,
    #[serde(alias = "y")]
    Vertical,
}

#[derive(specta::Type, Default)]
#[serde(rename_all = "camelCase")]
pub enum LinearDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(specta::Type)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SizedElement {
    Box(BoxElement),
    Margin(MarginElement),
    ContentBox(ContentBoxElement),
}

impl SizedElement {
    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        delegate_match! { match self {
            Self::{ Box, Margin, ContentBox }(e) => e.id(),
        }}
    }
}

#[derive(specta::Type)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LayoutElement {
    Box(BoxElement),
    Margin(MarginElement),
    ContentBox(ContentBoxElement),
    Array(ArrayElement),
    Align(AlignElement),
    Grid(GridElement),
    Flex(FlexElement),
}

impl LayoutElement {
    #[must_use]
    pub fn id(&self) -> Option<ElementId> {
        delegate_match! { match self {
            Self::{ Box, Margin, ContentBox, Array, Grid, Flex, Align }(e) => e.id(),
        }}
    }
}
